fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use tokio;

    const NFT_WASM_FILEPATH: &str = "./examples/counter.wasm";

    #[tokio::test]
    async fn test_nft_contract() -> anyhow::Result<()> {
        let worker = near_workspaces::sandbox().await?;
        let wasm = std::fs::read(NFT_WASM_FILEPATH)?;
        let contract = worker.dev_deploy(&wasm).await?;

        print!("Contract deployed at: {}", contract.id());

        Ok(())
    }
}
