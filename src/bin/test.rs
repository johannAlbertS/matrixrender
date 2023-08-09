use matrixrender;
//use matrixrender::Vectormap;

#[allow(non_upper_case_globals)]
static fonts: [&[&[bool]]; 26] = [
    [
        [false, true, true, true, true, true, true].as_slice(),//a
        [true, false, false, false, true, false, false].as_slice(),
        [true, false, false, false, true, false, false].as_slice(),
        [false, true, true, true, true, true, true].as_slice(),
    ].as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(),//b
        [true, false, false, true, false, false, true].as_slice(),
        [true, false, false, true, false, false, true].as_slice(),
        [false, true, true, false, true, true, false].as_slice(),
    ].as_slice(),
    [
        [false, true, true, true, true, true, false].as_slice(),//c
        [true, false, false, false, false, false, true].as_slice(),
        [true, false, false, false, false, false, true].as_slice(),
        [true, false, false, false, false, false, true].as_slice(),
    ].as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(),//d
        [true, false, false, false, false, false, true].as_slice(),
        [true, false, false, false, false, false, true].as_slice(),
        [false, true, true, true, true, true, false].as_slice(),
    ].as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(),//e
        [true, false, false, true, false, false, true].as_slice(),
        [true, false, false, true, false, false, true].as_slice(),
        [true, false, false, true, false, false, true].as_slice(),        
    ].as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(),//f
        [true, false, false, true, false, false, false].as_slice(),
        [true, false, false, true, false, false, false].as_slice(),
        [true, false, false, true, false, false, false].as_slice(),
    ].as_slice(),
    [
        [false, true, true, true, true, true, false].as_slice(),//g
        [true, false, false, false, false, false, true].as_slice(),
        [true, false, false, false, true, false, true].as_slice(),
        [true, false, false, false, true, true, false].as_slice()
    ].as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(),//h
        [false, false, false, true, false, false, false].as_slice(),
        [false, false, false, true, false, false, false].as_slice(),
        [true, true, true, true, true, true, true].as_slice(),
    ].as_slice(),
    [
        [true, false, false, false, false, false, true].as_slice(),//i
        [true, true, true, true, true, true, true].as_slice(),
        [true, false, false, false, false, false, true].as_slice(),
    ].as_slice(),
    [
        [true, false, false, false, false, true, false].as_slice(),//j
        [true, false, false, false, false, false, true].as_slice(),
        [true, false, false, false, false, false, true].as_slice(),
        [true, true, true, true, true, true, true].as_slice()
    ].as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(),
        [false, false, false, true, false, false, false].as_slice(),//k
        [false, false, true, false, true, false, false].as_slice(),
        [false, true, false, false, false, true, false].as_slice(),
        [true, false, false, false, false, false, true].as_slice(),
    ].as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(),//l
        [false, false, false, false, false, false, true].as_slice(),
        [false, false, false, false, false, false, true].as_slice(),
        [false, false, false, false, false, false, true].as_slice(),
    ].as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(),//m
        [false, true, false, false, false, false, false].as_slice(),
        [false, false, true, false, false, false, false].as_slice(),
        [false, true, false, false, false, false, false].as_slice(),
        [true, true, true, true, true, true, true].as_slice(),
    ].as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(),//n
        [false, true, false, false, false, false, false].as_slice(),
        [false, false, true, false, false, false, false].as_slice(),
        [false, false, false, true, false, false, false].as_slice(),
        [true, true, true, true, true, true, true].as_slice(),
    ].as_slice(),
    [
        [false, true, true, true, true, true, false].as_slice(),//o
        [true, false, false, false, false, false, false].as_slice(),
        [true, false, false, false, false, false, false].as_slice(),
        [false, true, true, true, true, true, false].as_slice(),
    ].as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(),//p
        [true, false, false, true, false, false, false].as_slice(),
        [true, false, false, true, false, false, false].as_slice(),
        [false, true, true, false, false, false, false].as_slice(),
    ].as_slice(),
    [
        [false, true, true, true, true, true, false].as_slice(),//q
        [true, false, false, false, false, false, true].as_slice(),
        [true, false, false, false, false, true, true].as_slice(),
        [false, true, true, true, true, true, false].as_slice(),
        [false, false, false, false, false, false, true].as_slice(),
    ].as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(),//r
        [true, false, false, true, true, false, false].as_slice(),
        [true, false, false, true, false, true, false].as_slice(),
        [false, true, true, false, false, false, true].as_slice(),
    ].as_slice(),
    [
        [false, true, true, false, false, false, true].as_slice(),//s
        [true, false, false, true, false, false, true].as_slice(),
        [true, false, false, true, false, false, true].as_slice(),
        [true, false, false, false, true, true, false].as_slice(),
    ].as_slice(),
    [
        [true, false, false, false, false, false, false].as_slice(),//t
        [true, false, false, false, false, false, false].as_slice(),
        [true, true, true, true, true, true, true].as_slice(),
        [true, false, false, false, false, false, false].as_slice(),
        [true, false, false, false, false, false, false].as_slice(),
    ].as_slice(),
    [
        [true, true, true, true, true, true, false].as_slice(),//u
        [false, false, false, false, false, false, true].as_slice(),
        [false, false, false, false, false, false, true].as_slice(),
        [true, true, true, true, true, true, false].as_slice(),
    ].as_slice(),
    [
        [true, true, true, true, true, false, false].as_slice(),//v
        [false, false, false, false, false, true, false].as_slice(),
        [false, false, false, false, false, false, true].as_slice(),
        [false, false, false, false, false, true, false].as_slice(),
        [true, true, true, true, true, false, false].as_slice(),
    ].as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(),//w
        [false, false, false, false, false, true, false].as_slice(),
        [false, false, false, false, true, false, false].as_slice(),
        [false, false, false, false, false, true, false].as_slice(),
        [true, true, true, true, true, true, true].as_slice(),
    ].as_slice(),
    [
        [true, true, false, false, false, true, true].as_slice(),//x
        [false, false, true, false, true, false, false].as_slice(),
        [false, false, false, true, false, false, false].as_slice(),
        [false, false, true, false, true, false, false].as_slice(),
        [true, true, false, false, false, true, true].as_slice(),
    ].as_slice(),
    [
        [true, true, false, false, false, false, false].as_slice(),//y
        [false, false, true, false, false, false, false].as_slice(),
        [false, false, false, true, true, true, true].as_slice(),
        [false, false, true, false, false, false, false].as_slice(),
        [true, true, false, false, false, false, false].as_slice(),
    ].as_slice(),
    [
        [true, false, false, false, false, false, true].as_slice(),//z
        [true, false, false, false, true, false, true].as_slice(),
        [true, false, false, true, false, false, true].as_slice(),
        [true, false, true, false, false, false, true].as_slice(),
        [true, true, false, false, false, false, true].as_slice(),
    ].as_slice(),
];

fn main() {
    /*let bitmap = [
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
                ((8, 0), (8, 6))
            ].as_slice()
        )
    ];
    matrixrender::draw_bitmap(&bitmap, (0, 0));
    matrixrender::draw_line((5, 0), (5, 5), (255, 255, 0));
    matrixrender::render_vectormap(vectormap);*/
    //matrixrender::fonts(&fonts);
    let mut text_buffer = [
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
        &mut [(0_u8,0_u8,0_u8), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0), (0,0,0),][..],
    ];
    matrixrender::fonts(&fonts);
    matrixrender::initialize_text_buffer(&mut text_buffer, &['H', 'E', 'L', 'L', 'O',], (255, 0, 0));
    matrixrender::draw_text_buffer(&mut text_buffer, 3);
    let display = matrixrender::buffer();
    for i in display {
        for j in i {
            print!("{:3?}", j);
        }
        print!("\n");
    }
    print!("\n\n");
    matrixrender::shift_text_buffer(&mut text_buffer);
    matrixrender::draw_text_buffer(&text_buffer, 3);
    let display = matrixrender::buffer();
    for i in display {
        for j in i {
            print!("{:3?}", j);
        }
        print!("\n");
    }
    print!("\n\n");
    matrixrender::shift_text_buffer(&mut text_buffer);
    matrixrender::draw_text_buffer(&text_buffer, 3);
    let display = matrixrender::buffer();
    for i in display {
        for j in i {
            print!("{:3?}", j);
        }
        print!("\n");
    }
    print!("\n\n");
    matrixrender::shift_text_buffer(&mut text_buffer);
    matrixrender::draw_text_buffer(&text_buffer, 3);
    let display = matrixrender::buffer();
    for i in display {
        for j in i {
            print!("{:3?}", j);
        }
        print!("\n");
    }
    print!("\n\n");
}
