cmake_minimum_required(VERSION 3.16)
project(chunk-your-skills-c VERSION 1.0.7 LANGUAGES C)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

get_filename_component(CYS_REPO_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../.." ABSOLUTE)
get_filename_component(CYS_INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/include" ABSOLUTE)
get_filename_component(CYS_BUILD_SCRIPT "${CMAKE_CURRENT_SOURCE_DIR}/scripts/build-c-lib.sh" ABSOLUTE)

option(CYS_BUILD_SHARED "Expose shared library imported target" ON)
option(CYS_BUILD_STATIC "Expose static library imported target" OFF)
option(CYS_BUILD_EXAMPLES "Build examples and register CTest" ON)
option(CYS_INSTALL "Generate install rules" ON)
option(CYS_BUILD_ALL_TARGETS "Build all six Rust triplets" OFF)

if(NOT DEFINED CYS_RUST_TARGET)
    execute_process(
        COMMAND rustc -vV
        OUTPUT_VARIABLE _cys_rustc_vv
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
    string(REGEX MATCH "host: ([^\n]+)" _cys_host_match "${_cys_rustc_vv}")
    set(CYS_RUST_TARGET "${CMAKE_MATCH_1}")
endif()

set(_cys_supported_targets
    x86_64-unknown-linux-gnu
    aarch64-unknown-linux-gnu
    x86_64-apple-darwin
    aarch64-apple-darwin
    x86_64-pc-windows-msvc
    aarch64-pc-windows-msvc
)
list(FIND _cys_supported_targets "${CYS_RUST_TARGET}" _cys_target_idx)
if(_cys_target_idx EQUAL -1)
    message(FATAL_ERROR "CYS_RUST_TARGET must be one of the supported triplets, got: ${CYS_RUST_TARGET}")
endif()

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CYS_LIB_PROFILE debug)
else()
    set(CYS_LIB_PROFILE release)
endif()

get_filename_component(CYS_TARGET_DIR "${CYS_REPO_ROOT}/target" ABSOLUTE)

get_filename_component(CYS_ARTIFACT_DIR "${CYS_TARGET_DIR}/${CYS_RUST_TARGET}/${CYS_LIB_PROFILE}" ABSOLUTE)

if(CYS_LIB_PROFILE STREQUAL "debug")
    set(_cys_build_profile_flag --debug)
else()
    set(_cys_build_profile_flag --release)
endif()

if(WIN32)
    set(CYS_SHARED_LIB "${CYS_ARTIFACT_DIR}/chunk_your_skills.dll")
    set(CYS_IMPORT_LIB "${CYS_ARTIFACT_DIR}/chunk_your_skills.dll.lib")
    set(CYS_STATIC_LIB "${CYS_ARTIFACT_DIR}/chunk_your_skills.lib")
else()
    if(APPLE)
        set(CYS_SHARED_LIB "${CYS_ARTIFACT_DIR}/libchunk_your_skills.dylib")
    else()
        set(CYS_SHARED_LIB "${CYS_ARTIFACT_DIR}/libchunk_your_skills.so")
    endif()
    set(CYS_STATIC_LIB "${CYS_ARTIFACT_DIR}/libchunk_your_skills.a")
endif()

add_custom_command(
    OUTPUT "${CYS_SHARED_LIB}"
    COMMAND bash "${CYS_BUILD_SCRIPT}" --target "${CYS_RUST_TARGET}" ${_cys_build_profile_flag}
    WORKING_DIRECTORY "${CYS_REPO_ROOT}"
    COMMENT "Building chunk-your-skills ffi for ${CYS_RUST_TARGET}"
    VERBATIM
)

add_custom_target(chunk_your_skills_build DEPENDS "${CYS_SHARED_LIB}")

if(CYS_BUILD_SHARED)
    add_library(chunk_your_skills SHARED IMPORTED GLOBAL)
    set_target_properties(chunk_your_skills PROPERTIES
        IMPORTED_LOCATION "${CYS_SHARED_LIB}"
        INTERFACE_INCLUDE_DIRECTORIES "${CYS_INCLUDE_DIR}"
    )
    if(WIN32)
        set_target_properties(chunk_your_skills PROPERTIES
            IMPORTED_IMPLIB "${CYS_IMPORT_LIB}"
        )
    endif()
    add_dependencies(chunk_your_skills chunk_your_skills_build)
    add_library(CYS::chunk_your_skills ALIAS chunk_your_skills)
endif()

if(CYS_BUILD_STATIC)
    add_library(chunk_your_skills_static STATIC IMPORTED GLOBAL)
    set_target_properties(chunk_your_skills_static PROPERTIES
        IMPORTED_LOCATION "${CYS_STATIC_LIB}"
        INTERFACE_INCLUDE_DIRECTORIES "${CYS_INCLUDE_DIR}"
    )
    add_dependencies(chunk_your_skills_static chunk_your_skills_build)
    add_library(CYS::chunk_your_skills_static ALIAS chunk_your_skills_static)
endif()

function(cys_add_example name source)
    add_executable("${name}" "${source}")
    target_include_directories("${name}" PRIVATE
        "${CMAKE_CURRENT_SOURCE_DIR}"
        "${CYS_INCLUDE_DIR}"
        "${CMAKE_CURRENT_SOURCE_DIR}/examples")
    if(WIN32)
        target_link_libraries("${name}" PRIVATE CYS::chunk_your_skills)
    else()
        # Static link avoids dyld picking up a stale shared lib when other SDK
        # builds (e.g. TypeScript napi) rebuild the crate with different features.
        target_link_libraries("${name}" PRIVATE "${CYS_STATIC_LIB}" pthread)
        if(APPLE)
            target_link_libraries("${name}" PRIVATE "-framework Security")
        else()
            target_link_libraries("${name}" PRIVATE dl m)
        endif()
    endif()
    add_dependencies("${name}" chunk_your_skills_build)
    if(WIN32)
        add_custom_command(TARGET "${name}" POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "${CYS_SHARED_LIB}" "$<TARGET_FILE_DIR:${name}>"
            COMMENT "Copy chunk_your_skills.dll beside ${name}"
        )
    endif()
endfunction()

if(CYS_BUILD_EXAMPLES)
    enable_testing()
    cys_add_example(cys_example_basic examples/basic.c)
    cys_add_example(cys_example_error_handling examples/error_handling.c)
    cys_add_example(cys_example_skills examples/skills.c)

    add_test(NAME cys_basic COMMAND cys_example_basic)
    add_test(NAME cys_error_handling COMMAND cys_example_error_handling)
    add_test(NAME cys_skills COMMAND cys_example_skills)
endif()

include(GNUInstallDirs)

if(CYS_INSTALL)
    install(FILES "${CYS_INCLUDE_DIR}/chunk_your_skills.h"
        DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}/chunk-your-skills"
    )
    if(CYS_BUILD_SHARED)
        install(FILES "${CYS_SHARED_LIB}"
            DESTINATION "${CMAKE_INSTALL_LIBDIR}/${CYS_RUST_TARGET}"
        )
        if(WIN32)
            install(FILES "${CYS_IMPORT_LIB}"
                DESTINATION "${CMAKE_INSTALL_LIBDIR}/${CYS_RUST_TARGET}"
            )
        endif()
    endif()
    if(CYS_BUILD_STATIC)
        install(FILES "${CYS_STATIC_LIB}"
            DESTINATION "${CMAKE_INSTALL_LIBDIR}/${CYS_RUST_TARGET}"
        )
    endif()

    include(CMakePackageConfigHelpers)
    configure_package_config_file(
        "${CMAKE_CURRENT_SOURCE_DIR}/cmake/CYSConfig.cmake.in"
        "${CMAKE_CURRENT_BINARY_DIR}/CYSConfig.cmake"
        INSTALL_DESTINATION "${CMAKE_INSTALL_LIBDIR}/cmake/CYS"
        PATH_VARS CYS_TARGET_DIR CYS_INCLUDE_DIR
    )
    write_basic_package_version_file(
        "${CMAKE_CURRENT_BINARY_DIR}/CYSConfigVersion.cmake"
        VERSION "${PROJECT_VERSION}"
        COMPATIBILITY SameMajorVersion
    )
    install(FILES
        "${CMAKE_CURRENT_BINARY_DIR}/CYSConfig.cmake"
        "${CMAKE_CURRENT_BINARY_DIR}/CYSConfigVersion.cmake"
        DESTINATION "${CMAKE_INSTALL_LIBDIR}/cmake/CYS"
    )
endif()
