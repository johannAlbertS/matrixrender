use matrixrender;
use matrixrender::Vectormap;

fn main() {
    let bitmap = [
        [(255_u8, 0_u8, 0_u8), (255, 0, 0), (255, 0, 0), (255, 0, 0), (255, 0, 0), (255, 0, 0), (255, 0, 0),].as_slice(),
        [(255, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (255, 0, 0), (0, 0, 0), (0, 0, 0),].as_slice(),
        [(255, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (255, 0, 0), (0, 0, 0), (0, 0, 0),].as_slice(),
        [(255, 0, 0), (255, 0, 0), (255, 0, 0), (255, 0, 0), (255, 0, 0), (255, 0, 0), (255, 0, 0),].as_slice(),
    ];
    let vectormap: Vectormap = &[
        (
            (255, 0, 255),
            [
                ((5, 7), (8, 7)),
                ((8, 0), (8, 5))
            ].as_slice()
        )
    ];
    matrixrender::draw_bitmap(&bitmap, (0, 0));
    matrixrender::draw_line((5, 0), (5, 5), (255, 255, 0));
    matrixrender::render_vectormap(vectormap);
    let display = matrixrender::buffer();
    for i in display {
        for j in i {
            print!("{:3?}", j);
        }
        print!("\n");
    }
}