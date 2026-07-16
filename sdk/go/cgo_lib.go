package chunkyourskills

//go:generate go run ./cmd/chunk-native-ensure

/*
#cgo CFLAGS: -I${SRCDIR}/../c/include
#cgo linux,amd64 LDFLAGS: -L${SRCDIR}/native/x86_64-unknown-linux-gnu -L${SRCDIR}/../../target/x86_64-unknown-linux-gnu/release -lchunk_your_skills -lm -ldl -pthread
#cgo linux,arm64 LDFLAGS: -L${SRCDIR}/native/aarch64-unknown-linux-gnu -L${SRCDIR}/../../target/aarch64-unknown-linux-gnu/release -lchunk_your_skills -lm -ldl -pthread
#cgo darwin,amd64 LDFLAGS: -L${SRCDIR}/native/x86_64-apple-darwin -L${SRCDIR}/../../target/x86_64-apple-darwin/release -lchunk_your_skills -framework Security -lpthread
#cgo darwin,arm64 LDFLAGS: -L${SRCDIR}/native/aarch64-apple-darwin -L${SRCDIR}/../../target/aarch64-apple-darwin/release -lchunk_your_skills -framework Security -lpthread
#cgo windows,amd64 LDFLAGS: ${SRCDIR}/native/x86_64-pc-windows-msvc/libchunk_your_skills.a
#cgo windows,arm64 LDFLAGS: ${SRCDIR}/native/aarch64-pc-windows-msvc/libchunk_your_skills.a
#ifdef index
#undef index
#endif
#include "chunk_your_skills.h"
#include <stdlib.h>
*/
import "C"

import (
	"errors"
	"fmt"
	"unsafe"
)

const ok = 0

type skillsBuilderHandle struct {
	h *C.CYT_CytSkillsBuilder
}

func lastError() error {
	msg := C.cyt_get_last_error()
	if msg == nil {
		return errors.New("chunk-your-skills error")
	}
	return errors.New(C.GoString(msg))
}

func cString(s string) *C.char {
	return C.CString(s)
}

func freeCString(s *C.char) {
	C.free(unsafe.Pointer(s))
}

func takeJSON(out **C.char) (string, error) {
	if out == nil {
		return "", errors.New("null out pointer")
	}
	ptr := *out
	*out = nil
	if ptr == nil {
		return "", errors.New("null JSON output")
	}
	defer C.cyt_free_string(ptr)
	return C.GoString(ptr), nil
}

func cgoClearError() {
	C.cyt_clear_error()
}

func cgoGetLastError() string {
	msg := C.cyt_get_last_error()
	if msg == nil {
		return ""
	}
	return C.GoString(msg)
}

func cgoGetVersion() (string, error) {
	var out *C.char
	if C.cyt_get_version(&out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoCountTokens(text string) (int64, error) {
	cText := cString(text)
	defer freeCString(cText)
	count := C.cyt_count_tokens(cText)
	if count < 0 {
		return 0, lastError()
	}
	return int64(count), nil
}

func cgoCountJsonTokens(jsonStr string) (int64, error) {
	cJSON := cString(jsonStr)
	defer freeCString(cJSON)
	count := C.cyt_count_json_tokens(cJSON)
	if count < 0 {
		return 0, lastError()
	}
	return int64(count), nil
}

func cgoConfigureTokenizerDefaults(configJSON string) error {
	var cCfg *C.char
	if configJSON != "" {
		cCfg = cString(configJSON)
		defer freeCString(cCfg)
	}
	if C.cyt_configure_tokenizer_defaults(cCfg) != ok {
		return lastError()
	}
	return nil
}

func cgoCountTokensBatch(textsJSON string) (string, error) {
	cTexts := cString(textsJSON)
	defer freeCString(cTexts)
	var out *C.char
	if C.cyt_count_tokens_batch(cTexts, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoSkillsBuilderNew(memoryOnly bool, outputDir string) (skillsBuilderHandle, error) {
	cDir := cString(outputDir)
	defer freeCString(cDir)
	mem := C.int(0)
	if memoryOnly {
		mem = 1
	}
	var handle *C.CYT_CytSkillsBuilder
	if C.cyt_skills_builder_new(mem, cDir, &handle) != ok {
		return skillsBuilderHandle{}, lastError()
	}
	return skillsBuilderHandle{h: handle}, nil
}

func cgoSkillsBuilderFree(h skillsBuilderHandle) {
	if h.h != nil {
		C.cyt_skills_builder_free(h.h)
	}
}

func cgoSkillsBuilderBuildFromDirs(h skillsBuilderHandle, skillDirsJSON, configJSON string) (string, error) {
	cDirs := cString(skillDirsJSON)
	defer freeCString(cDirs)
	cCfg := cString(configJSON)
	defer freeCString(cCfg)
	var out *C.char
	if C.cyt_skills_builder_build_from_dirs(h.h, cDirs, cCfg, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoSkillsBuilderWriteCatalog(h skillsBuilderHandle) (string, error) {
	var out *C.char
	if C.cyt_skills_builder_write_catalog(h.h, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoSkillsBuilderToSkillsIndexJSON(h skillsBuilderHandle) (string, error) {
	var out *C.char
	if C.cyt_skills_builder_to_skills_index_json(h.h, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoSkillsBuilderToSkillsDict(h skillsBuilderHandle) (string, error) {
	var out *C.char
	if C.cyt_skills_builder_to_skills_dict(h.h, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoBuildSkillsIndex(skillDirsJSON, configJSON string) (string, error) {
	cDirs := cString(skillDirsJSON)
	defer freeCString(cDirs)
	cCfg := cString(configJSON)
	defer freeCString(cCfg)
	var out *C.char
	if C.cyt_build_skills_index(cDirs, cCfg, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoWriteSkillsIndex(indexJSON, outputDir string) error {
	cIndex := cString(indexJSON)
	defer freeCString(cIndex)
	cDir := cString(outputDir)
	defer freeCString(cDir)
	if C.cyt_write_skills_index(cIndex, cDir) != ok {
		return lastError()
	}
	return nil
}

func cgoLoadSkillsIndexFromDir(catalogDir string) (string, error) {
	cDir := cString(catalogDir)
	defer freeCString(cDir)
	var out *C.char
	if C.cyt_load_skills_index_from_dir(cDir, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoRepairSkillChunks(entryDir, docID, configJSON string) error {
	cDir := cString(entryDir)
	defer freeCString(cDir)
	cDoc := cString(docID)
	defer freeCString(cDoc)
	cCfg := cString(configJSON)
	defer freeCString(cCfg)
	if C.cyt_repair_skill_nodes(cDir, cDoc, cCfg) != ok {
		return lastError()
	}
	return nil
}

func cgoSkillsIndexFromDecomposedDir(dir string) (string, error) {
	cDir := cString(dir)
	defer freeCString(cDir)
	var out *C.char
	if C.cyt_skills_index_from_decomposed_dir(cDir, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoMdToTree(markdownContent, sourcePath, configJSON string) (string, error) {
	cMd := cString(markdownContent)
	defer freeCString(cMd)
	cPath := cString(sourcePath)
	defer freeCString(cPath)
	cCfg := cString(configJSON)
	defer freeCString(cCfg)
	var out *C.char
	if C.cyt_md_to_tree(cMd, cPath, cCfg, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoGetSkillDocument(documentsJSON, docID string) (string, error) {
	cDocs := cString(documentsJSON)
	defer freeCString(cDocs)
	cDoc := cString(docID)
	defer freeCString(cDoc)
	var out *C.char
	if C.cyt_get_skill_document(cDocs, cDoc, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoGetSkillStructure(documentsJSON, docID string) (string, error) {
	cDocs := cString(documentsJSON)
	defer freeCString(cDocs)
	cDoc := cString(docID)
	defer freeCString(cDoc)
	var out *C.char
	if C.cyt_get_skill_structure(cDocs, cDoc, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoGetSkillLineContentFromSpec(indexOrDocsJSON, docID, lineNumSpec string) (string, error) {
	cIndex := cString(indexOrDocsJSON)
	defer freeCString(cIndex)
	cDoc := cString(docID)
	defer freeCString(cDoc)
	cSpec := cString(lineNumSpec)
	defer freeCString(cSpec)
	var out *C.char
	if C.cyt_get_skill_line_content_from_spec(cIndex, cDoc, cSpec, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoGetSkillContentRetrieveResult(indexOrDocsJSON, docID, lineNumSpecsJSON, nodeIDSpecsJSON, _, optionsJSON string) (string, error) {
	cIndex := cString(indexOrDocsJSON)
	defer freeCString(cIndex)
	cDoc := cString(docID)
	defer freeCString(cDoc)
	cLines := cString(lineNumSpecsJSON)
	defer freeCString(cLines)
	cNodes := cString(nodeIDSpecsJSON)
	defer freeCString(cNodes)
	cOpts := cString(optionsJSON)
	defer freeCString(cOpts)
	var out *C.char
	if C.cyt_get_skill_content_retrieve_result(cIndex, cDoc, cLines, cNodes, cOpts, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoReconstructSkillMarkdown(indexOrDocsJSON, docID, lineNumSpecsJSON, nodeIDSpecsJSON, _, optionsJSON string) (string, error) {
	cIndex := cString(indexOrDocsJSON)
	defer freeCString(cIndex)
	cDoc := cString(docID)
	defer freeCString(cDoc)
	cLines := cString(lineNumSpecsJSON)
	defer freeCString(cLines)
	cNodes := cString(nodeIDSpecsJSON)
	defer freeCString(cNodes)
	cOpts := cString(optionsJSON)
	defer freeCString(cOpts)
	var out *C.char
	if C.cyt_reconstruct_skill_markdown(cIndex, cDoc, cLines, cNodes, cOpts, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoWriteReconstructedSkill(catalogDir, indexOrDocsJSON, docID, lineNumSpecsJSON, nodeIDSpecsJSON, _, optionsJSON string) error {
	cDir := cString(catalogDir)
	defer freeCString(cDir)
	cIndex := cString(indexOrDocsJSON)
	defer freeCString(cIndex)
	cDoc := cString(docID)
	defer freeCString(cDoc)
	cLines := cString(lineNumSpecsJSON)
	defer freeCString(cLines)
	cNodes := cString(nodeIDSpecsJSON)
	defer freeCString(cNodes)
	cOpts := cString(optionsJSON)
	defer freeCString(cOpts)
	var out *C.char
	if C.cyt_write_reconstructed_skill(cDir, cIndex, cDoc, cLines, cNodes, cOpts, &out) != ok {
		return lastError()
	}
	_, err := takeJSON(&out)
	return err
}

func cgoGetSkillLineContent(indexOrDocsJSON, docID, lineNumSpecsJSON, nodeIDSpecsJSON, _ string) (string, error) {
	cIndex := cString(indexOrDocsJSON)
	defer freeCString(cIndex)
	cDoc := cString(docID)
	defer freeCString(cDoc)
	cLines := cString(lineNumSpecsJSON)
	defer freeCString(cLines)
	cNodes := cString(nodeIDSpecsJSON)
	defer freeCString(cNodes)
	var out *C.char
	if C.cyt_get_skill_line_content(cIndex, cDoc, cLines, cNodes, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoParseSkillChunkIDs(spec string) (string, error) {
	return cgoParseSkillNodeIDs(spec)
}

func cgoParseSkillNodeIDs(spec string) (string, error) {
	cSpec := cString(spec)
	defer freeCString(cSpec)
	var out *C.char
	if C.cyt_parse_skill_node_ids(cSpec, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoTokenCountFromDecomposedFrontmatter(content string) (int64, bool, error) {
	cContent := cString(content)
	defer freeCString(cContent)
	var out C.long
	if C.cyt_token_count_from_decomposed_frontmatter(cContent, &out) != ok {
		return 0, false, lastError()
	}
	if out < 0 {
		return 0, false, nil
	}
	return int64(out), true, nil
}

func cgoParseFrontmatterFields(content string) (string, error) {
	cContent := cString(content)
	defer freeCString(cContent)
	var out *C.char
	if C.cyt_parse_frontmatter_fields(cContent, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoFrontmatterField(content, key string) (string, error) {
	cContent := cString(content)
	defer freeCString(cContent)
	cKey := cString(key)
	defer freeCString(cKey)
	var out *C.char
	if C.cyt_frontmatter_field(cContent, cKey, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoReconstructOptionsDefault() (string, error) {
	var out *C.char
	if C.cyt_reconstruct_options_default(&out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoEnsureSkillsRegistry(sourcePathsJSON, catalogRoot, pageindexConfigJSON, policy string) (string, error) {
	cPaths := cString(sourcePathsJSON)
	defer freeCString(cPaths)
	cRoot := cString(catalogRoot)
	defer freeCString(cRoot)
	var cCfg *C.char
	if pageindexConfigJSON != "" {
		cCfg = cString(pageindexConfigJSON)
		defer freeCString(cCfg)
	}
	var cPolicy *C.char
	if policy != "" {
		cPolicy = cString(policy)
		defer freeCString(cPolicy)
	}
	var out *C.char
	if C.cyt_ensure_skills_registry(cPaths, cRoot, cCfg, cPolicy, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoBuildPageIndexOnly(skillDirsJSON, configJSON string) (string, error) {
	cDirs := cString(skillDirsJSON)
	defer freeCString(cDirs)
	cCfg := cString(configJSON)
	defer freeCString(cCfg)
	var out *C.char
	if C.cyt_build_page_index_only(cDirs, cCfg, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoPageIndexValid(entryDir, contentSHA256 string) (bool, error) {
	cEntry := cString(entryDir)
	defer freeCString(cEntry)
	cHash := cString(contentSHA256)
	defer freeCString(cHash)
	var out C.int
	if C.cyt_page_index_valid(cEntry, cHash, &out) != ok {
		return false, lastError()
	}
	return out != 0, nil
}

func cgoLoadSkillsIndexFromEntry(entryDir, docID, _ string) (string, error) {
	cEntry := cString(entryDir)
	defer freeCString(cEntry)
	cDoc := cString(docID)
	defer freeCString(cDoc)
	var out *C.char
	if C.cyt_load_skills_index_from_entry(cEntry, cDoc, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoLoadMergedSkillDocumentJSON(entryDir, docID, _ string) (string, error) {
	cEntry := cString(entryDir)
	defer freeCString(cEntry)
	cDoc := cString(docID)
	defer freeCString(cDoc)
	var out *C.char
	if C.cyt_load_merged_skill_document_json(cEntry, cDoc, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoFinalizeSkillDocumentJSON(entryDir, docID, contentSHA256, pipeline, indexParamsJSON, builtAt, sourcePath string) (string, error) {
	cEntry := cString(entryDir)
	defer freeCString(cEntry)
	cDoc := cString(docID)
	defer freeCString(cDoc)
	cHash := cString(contentSHA256)
	defer freeCString(cHash)
	cPipeline := cString(pipeline)
	defer freeCString(cPipeline)
	cParams := cString(indexParamsJSON)
	defer freeCString(cParams)
	cBuilt := cString(builtAt)
	defer freeCString(cBuilt)
	cSource := cString(sourcePath)
	defer freeCString(cSource)
	var out *C.char
	if C.cyt_finalize_skill_document_json(cEntry, cDoc, cHash, cPipeline, cParams, cBuilt, cSource, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoUpdateSkillDocumentSourcePath(entryDir, docID, sourcePath string) (string, error) {
	cEntry := cString(entryDir)
	defer freeCString(cEntry)
	cDoc := cString(docID)
	defer freeCString(cDoc)
	cSource := cString(sourcePath)
	defer freeCString(cSource)
	var out *C.char
	if C.cyt_update_skill_document_source_path(cEntry, cDoc, cSource, &out) != ok {
		return "", lastError()
	}
	return takeJSON(&out)
}

func cgoConfigureMemoryCache(configJSON string) error {
	var cCfg *C.char
	if configJSON != "" {
		cCfg = cString(configJSON)
		defer freeCString(cCfg)
	}
	if C.cyt_configure_memory_cache(cCfg) != ok {
		return lastError()
	}
	return nil
}

func cgoBuildChunkVariant(_, _, _, _, _ string) (string, error) {
	return "", fmt.Errorf("chunk variants were removed from chunk-your-skills; use BuildPageIndexOnly instead")
}

func cgoChunkVariantValid(_, _, _, _ string) (bool, error) {
	return false, nil
}

func cgoRepairSkillVariantChunks(entryDir, docID, _, _, configJSON string) error {
	return cgoRepairSkillChunks(entryDir, docID, configJSON)
}
