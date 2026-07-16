use chunk_your_skills::{
    PageIndexConfig, ReconstructOptions, SkillsBuilder, load_skills_index_from_dir,
    reconstruct_skill_markdown, resolve_doc_id, write_reconstructed_skill,
};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    name = "chunk-your-skills",
    version = env!("CARGO_PKG_VERSION"),
    about = "Chunk/Decompose SKILL.md into page-indexed nodes and recompose skinny skills"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Decompose a SKILL.md file into metadata and node files under a catalog directory
    Decompose {
        #[arg(long)]
        skill: PathBuf,
        #[arg(long, default_value = ".catalog")]
        output: PathBuf,
    },
    /// Recompose a skinny SKILL.md from selected node IDs
    Recompose {
        /// Decomposed catalog directory (from `decompose`)
        #[arg(long, group = "source")]
        catalog: Option<PathBuf>,
        /// SKILL.md file to index in memory (no catalog required)
        #[arg(long, group = "source")]
        skill: Option<PathBuf>,
        /// Catalog document id (from `page_index.json` `id` field)
        #[arg(long, group = "doc_selector", requires = "catalog")]
        doc_id: Option<String>,
        /// Original skill file path (from `page_index.json` `path` field)
        #[arg(long, group = "doc_selector", requires = "catalog")]
        path: Option<PathBuf>,
        /// Node ids: ranges, lists, or both (e.g. `1-3,5,8`)
        #[arg(long, value_delimiter = ',')]
        node_id: Vec<String>,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long, default_value_t = false)]
        keep_all_headers: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Decompose { skill, output } => run_decompose(&skill, &output)?,
        Commands::Recompose {
            catalog,
            skill,
            doc_id,
            path,
            node_id,
            output,
            keep_all_headers,
        } => match (catalog, skill) {
            (Some(catalog), None) => run_recompose_from_catalog(
                &catalog,
                doc_id.as_deref(),
                path.as_deref(),
                &node_id,
                output.as_deref(),
                keep_all_headers,
            )?,
            (None, Some(skill)) => {
                run_recompose_from_skill(&skill, &node_id, output.as_deref(), keep_all_headers)?;
            }
            _ => unreachable!("clap source group requires exactly one of --catalog or --skill"),
        },
    }
    Ok(())
}

fn run_decompose(skill: &Path, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !skill.is_file() {
        return Err(format!("skill file not found: {}", skill.display()).into());
    }

    let mut builder = SkillsBuilder::new(false, Some(output.to_path_buf()));
    builder.build_from_file(skill, &PageIndexConfig::default())?;
    builder.write_catalog()?;

    let index = builder
        .index()
        .ok_or("internal error: missing skills index")?;
    let doc_id = index
        .documents
        .keys()
        .next()
        .cloned()
        .ok_or("no document indexed from skill file")?;

    eprintln!(
        "Wrote catalog for doc_id={doc_id} under {}",
        output.display()
    );
    Ok(())
}

fn run_recompose_from_skill(
    skill: &Path,
    node_ids: &[String],
    output: Option<&Path>,
    keep_all_headers: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if !skill.is_file() {
        return Err(format!("skill file not found: {}", skill.display()).into());
    }
    let output = output.ok_or("--output is required when using --skill")?;

    let mut builder = SkillsBuilder::new(true, None);
    builder.build_from_file(skill, &PageIndexConfig::default())?;
    let index = builder
        .index()
        .ok_or("internal error: missing skills index")?;
    let doc_id = index
        .documents
        .keys()
        .next()
        .cloned()
        .ok_or("no document indexed from skill file")?;

    let node_id_specs: Vec<&str> = node_ids.iter().map(String::as_str).collect();
    let opts = ReconstructOptions { keep_all_headers };
    let result = reconstruct_skill_markdown(index, &doc_id, &[], &node_id_specs, &opts)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output, &result.markdown)?;
    eprintln!("Wrote skinny skill to {}", output.display());
    Ok(())
}

fn run_recompose_from_catalog(
    catalog: &Path,
    doc_id: Option<&str>,
    skill_path: Option<&Path>,
    node_ids: &[String],
    output: Option<&Path>,
    keep_all_headers: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let resolved_doc_id = resolve_doc_id(catalog, doc_id, skill_path)?;
    let index = load_skills_index_from_dir(catalog)?;
    let node_id_specs: Vec<&str> = node_ids.iter().map(String::as_str).collect();
    let opts = ReconstructOptions { keep_all_headers };

    if let Some(out) = output {
        let result =
            reconstruct_skill_markdown(&index, &resolved_doc_id, &[], &node_id_specs, &opts)?;
        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(out, &result.markdown)?;
        eprintln!("Wrote skinny skill to {}", out.display());
    } else {
        let reconstructed = write_reconstructed_skill(
            catalog,
            &index,
            &resolved_doc_id,
            &[],
            &node_id_specs,
            &opts,
        )?;
        eprintln!("Wrote skinny skill to {}", reconstructed.display());
    }

    Ok(())
}
