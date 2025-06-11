use criterion::{Criterion, criterion_group, criterion_main};

use async_trait::async_trait;
use pumpkin_data::BlockDirection;
use pumpkin_util::math::position::BlockPos;
use pumpkin_util::math::vector2::Vector2;
use std::sync::Arc;
use temp_dir::TempDir;

use pumpkin_world::dimension::Dimension;
use pumpkin_world::generation::{Seed, get_world_gen};
use pumpkin_world::level::Level;
use pumpkin_world::world::{BlockAccessor, BlockRegistryExt};

use tokio::runtime::Runtime;

struct BlockRegistry;

#[async_trait]
impl BlockRegistryExt for BlockRegistry {
    async fn can_place_at(
        &self,
        _block: &pumpkin_data::Block,
        _block_accessor: &dyn BlockAccessor,
        _block_pos: &BlockPos,
        _face: BlockDirection,
    ) -> bool {
        true
    }
}

fn bench_chunk_generation(c: &mut Criterion) {
    let seed = 0;
    let generator = get_world_gen(Seed(seed), Dimension::Overworld);
    let temp_dir = TempDir::new().unwrap();
    let block_registry = Arc::new(BlockRegistry);
    let level = Arc::new(Level::from_root_folder(
        temp_dir.path().to_path_buf(),
        block_registry.clone(),
        seed.try_into().unwrap(),
        Dimension::Overworld,
    ));
    let x = 0;
    let y = 0;
    let position = Vector2::new(x, y);
    let runtime = Runtime::new().unwrap();
    c.bench_function("chunk generation", |b| {
        b.to_async(&runtime)
            .iter(|| generator.generate_chunk(&level, block_registry.as_ref(), &position))
    });
}

criterion_group!(benches, bench_chunk_generation);
criterion_main!(benches);
