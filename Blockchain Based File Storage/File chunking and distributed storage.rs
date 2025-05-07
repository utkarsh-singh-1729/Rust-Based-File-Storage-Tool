use std::fs::{self, File};
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

// Chunk metadata structure
#[derive(Debug, Serialize, Deserialize)]
struct ChunkMetadata {
    chunk_number: u32,
    uuid: String,
    node: String,
    original_filename: String,
}

// Create storage node directories
fn create_storage_nodes(storage_nodes: &[String]) -> std::io::Result<()> {
    for node in storage_nodes {
        fs::create_dir_all(node)?;
    }
    Ok(())
}

// Split file into chunks and distribute across nodes
fn chunk_file(
    input_path: &Path,
    chunk_size: usize,
    storage_nodes: &[String],
    manifest_path: &Path,
) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);
    let mut manifest = Vec::new();
    let original_filename = input_path.file_name().unwrap().to_str().unwrap();

    let mut chunk_number = 0;
    let node_count = storage_nodes.len();
    let mut buffer = vec![0; chunk_size];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        // Select storage node in round-robin fashion
        let node = &storage_nodes[chunk_number % node_count];
        let uuid = Uuid::new_v4().to_string();
        let chunk_path = PathBuf::from(node).join(&uuid);

        // Write chunk to selected node
        let mut chunk_file = BufWriter::new(File::create(&chunk_path)?);
        chunk_file.write_all(&buffer[..bytes_read])?;

        // Record metadata
        manifest.push(ChunkMetadata {
            chunk_number,
            uuid: uuid.clone(),
            node: node.to_string(),
            original_filename: original_filename.to_string(),
        });

        chunk_number += 1;
    }

    // Save manifest file
    let manifest_file = File::create(manifest_path)?;
    serde_json::to_writer(manifest_file, &manifest)?;

    Ok(())
}

// Reconstruct file from chunks using manifest
fn reconstruct_file(
    manifest_path: &Path,
    output_path: &Path,
) -> std::io::Result<()> {
    let manifest_file = File::open(manifest_path)?;
    let mut manifest: Vec<ChunkMetadata> = serde_json::from_reader(manifest_file)?;

    // Sort chunks by their sequence number
    manifest.sort_by_key(|c| c.chunk_number);

    let mut output_file = BufWriter::new(File::create(output_path)?);

    for chunk in manifest {
        let chunk_path = PathBuf::from(&chunk.node).join(&chunk.uuid);
        let mut chunk_file = BufReader::new(File::open(chunk_path)?);
        let mut buffer = Vec::new();
        chunk_file.read_to_end(&mut buffer)?;
        output_file.write_all(&buffer)?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    // Example configuration
    let storage_nodes = vec![
        "storage/node0".to_string(),
        "storage/node1".to_string(),
        "storage/node2".to_string(),
    ];

    // Create storage directories
    create_storage_nodes(&storage_nodes)?;

    // Split input file into chunks
    chunk_file(
        Path::new("input.txt"),
        1024 * 1024 * 1024,  //1GB chunks
        &storage_nodes,
        Path::new("manifest.json"),
    )?;

    // Reconstruct file from chunks
    reconstruct_file(
        Path::new("manifest.json"),
        Path::new("output.txt"),
    )?;

    Ok(())
}