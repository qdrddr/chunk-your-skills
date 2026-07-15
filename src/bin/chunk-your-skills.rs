use chunk_your_skills::{
    PageIndexConfig, ReconstructOptions, SkillsBuilder, load_skills_index_from_dir,
    write_reconstructed_skill,
};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "chunk-your-skills")]
#[command(about = "Decompose SKILL.md into page-indexed nodes and recompose skinny skills")]
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
        #[arg(long)]
        catalog: PathBuf,
        #[arg(long)]
        doc_id: String,
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
            doc_id,
            node_id,
            output,
            keep_all_headers,
        } => run_recompose(
            &catalog,
            &doc_id,
            &node_id,
            output.as_deref(),
            keep_all_headers,
        )?,
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

fn run_recompose(
    catalog: &Path,
    doc_id: &str,
    node_ids: &[String],
    output: Option<&Path>,
    keep_all_headers: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = load_skills_index_from_dir(catalog)?;
    let node_id_specs: Vec<&str> = node_ids.iter().map(String::as_str).collect();
    let opts = ReconstructOptions { keep_all_headers };

    let reconstructed =
        write_reconstructed_skill(catalog, &index, doc_id, &[], &node_id_specs, &opts)?;

    if let Some(out) = output {
        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&reconstructed, out)?;
        eprintln!("Wrote skinny skill to {}", out.display());
    } else {
        eprintln!("Wrote skinny skill to {}", reconstructed.display());
    }

    Ok(())
}
