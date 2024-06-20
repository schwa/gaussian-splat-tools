test:
    cargo install --path .
    gaussian-splat-tools info -i TestData/OneSplat.splat
    gaussian-splat-tools guess-format -i TestData/test-splat.3-points-from-train.ply
    gaussian-splat-tools info -i TestData/test-splat.3-points-from-train.ply
    gaussian-splat-tools convert -i TestData/test-splat.3-points-from-train.ply -o test.json
    gaussian-splat-tools convert -i TestData/test-splat.3-points-from-train.ply -o test.splat
    gaussian-splat-tools convert -i TestData/test-splat.3-points-from-train.ply -o test.splatc
    gaussian-splat-tools dump -i TestData/test-splat.3-points-from-train.ply
