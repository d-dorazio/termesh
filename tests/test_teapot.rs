use std::io;

use termesh::drawille::Canvas;
use termesh::stl::Stl;

#[test]
fn test_teapot() {
    let teapot_stl = include_bytes!("../data/teapot.stl");

    let stl = Stl::parse_binary(&mut io::Cursor::new(&teapot_stl[..])).unwrap();

    let mut canvas = Canvas::new();
    let scale = 40.0;

    for facet in stl.facets {
        canvas.triangle(
            facet.vertices[0] * scale,
            facet.vertices[1] * scale,
            facet.vertices[2] * scale,
        );
    }

    assert_eq!(canvas.rows().collect::<Vec<_>>(), vec![
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣤⣤⣤⣴⣶⣶⡾⠿⡿⣿⣶⣶⣶⣤⣤⣤⣄⣀⣀⡀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⣾⣿⡿⣿⣿⣿⣿⠿⣯⣽⣲⣶⣚⣋⣏⡙⠛⠛⠻⠭⣽⣿⣛⣛⣛⠿⣿⣻⢶⣤⣀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⢾⣿⠟⣋⡥⠖⣯⣉⣤⣤⣴⣒⡻⠶⠭⣭⡽⠖⡗⠛⠿⠿⢷⡶⣛⣫⣽⣿⢭⣩⡟⠺⢿⣾⣽⣻⣶⣤⣀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⢾⡻⣝⣞⠯⢶⣟⢾⡻⡽⠚⣏⠁⢈⣉⣭⣝⣯⣭⣵⣶⡶⡷⠶⣤⣶⣶⣞⣓⣒⣶⣉⢉⡟⠛⢷⣦⢌⣑⠮⣝⡛⠿⣿⣶⣤⡀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⣪⡾⢾⡭⠛⣉⣤⣲⣝⣮⡥⢞⣲⣾⠿⡛⠋⠙⠛⢛⣲⣖⣪⡥⠤⡧⣤⣀⣈⣑⡺⠭⣭⡭⠛⡻⠷⣤⣤⣈⣫⡚⠽⣫⢾⣳⠦⣭⢿⡪⣦⡀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⡿⢋⡴⣣⢽⣟⠋⠉⢀⡴⣕⣞⣋⠵⣥⠶⣷⣲⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣿⣿⣿⣿⣭⣽⣳⣧⢄⡀⠙⢟⢖⡭⣫⢖⠛⢾⣽⣅⠙⢿⣪⡻⣦⡀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⡾⣫⢎⡴⣕⢝⢕⠏⣙⢶⠾⠓⠉⣩⣮⣷⣽⣶⣿⣿⣿⣿⣿⣿⣿⠿⠿⠿⠛⡿⣻⠿⠿⢿⣿⣿⣿⣿⣿⣻⢿⣯⣗⣶⣗⡬⡵⢯⣗⢞⠁⠙⣗⢄⡙⣝⠮⣻⣦⡀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⡾⣫⣺⠗⢩⢞⢕⣡⢚⢝⢵⢋⡳⣶⣯⣿⣿⣿⣿⡿⠟⠋⣹⣉⠶⠦⢤⣒⠶⠖⠋⡏⠑⠓⢖⣢⣤⣤⣌⣹⡙⡟⠿⣿⣿⣿⣿⣿⣽⡶⡋⠙⢗⢤⡈⢗⢝⢮⢗⢌⠻⣯⡢⡀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⡾⣞⠟⢁⣴⠵⠋⣡⢗⣕⡵⣫⣯⣮⢟⢟⣮⣗⠋⣁⠤⡲⠟⣉⣧⠴⠞⠛⢒⣛⣛⠿⡯⢍⣛⣛⠛⠲⣤⢤⣯⣈⣙⢦⢄⡉⠛⠿⣿⢟⢯⣻⣦⣀⢳⢝⢝⢢⣑⢕⢯⡳⢌⣿⣺⣆",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⡿⢡⣏⣗⣿⣅⢀⣴⠕⠉⣰⣿⣿⣿⣷⠟⠁⡣⡒⢉⣭⠯⠛⣙⡶⠻⣗⠉⠉⡡⠖⠁⠀⡟⡄⣀⠤⠛⠉⢑⣿⠢⢌⡑⠫⢏⡚⠓⡦⡛⡕⢵⣿⣿⣿⣯⡳⣕⣕⢧⢉⣷⢷⡏⢣⡷⣻⣆",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⢡⡻⣞⢾⢁⡼⣵⢧⡠⡾⣻⢿⣿⠟⠁⡠⡺⢁⢽⣍⠤⠒⠉⡰⠁⠀⢻⣵⣮⠤⠤⠴⢲⡗⠻⡤⠤⢤⣀⣼⣻⡤⠤⠬⠵⠾⢞⡟⢥⣈⢣⡀⠙⢿⣿⢿⡯⡳⣨⠗⣿⡀⠈⣿⡄⢻⢵⣻⣆",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣻⢣⢏⣟⠎⡮⡳⡝⡧⣞⣿⣿⣕⣕⠥⡤⢚⢞⠞⢉⡟⠉⢫⡒⡴⣁⠤⠒⢹⢷⠀⠑⢄⡰⠁⡇⠀⠘⣄⠔⢺⠏⠑⡧⢄⡀⠀⣠⡟⠑⢄⠑⢽⡪⣢⡀⠙⢗⢽⣿⡿⡄⠘⡵⣄⢸⢞⣄⣯⢳⣿⣆",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣳⣿⠃⣜⡮⢺⣱⢝⢴⣽⣿⣿⣿⠏⢩⠮⣄⠞⢁⠔⠁⡇⠀⢀⢽⢍⠀⠑⠢⣎⡘⡆⢀⠎⠑⢄⡇⡠⠔⠙⡄⡞⠀⠀⣇⡠⢬⠟⢗⠗⠤⠤⠵⢤⣗⣄⡭⢪⢹⢯⢾⣿⣿⡦⣻⢵⡹⡏⢞⣽⣣⠹⣿⣆",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣳⣻⠃⣼⠟⠀⣾⠟⠁⡼⣽⣿⣿⠏⢠⡻⢠⣻⡝⠷⣤⢴⢅⣔⣁⢸⠈⠢⡀⢰⣁⢬⣻⡷⠶⠛⢛⡟⠛⠳⠶⣾⣷⢖⡉⢹⡠⠊⠀⢸⠑⢄⠀⢀⡠⡟⡅⢳⡄⣧⠈⢿⣿⣿⣟⡝⣌⢷⣹⠪⣞⡏⢧⢹⣿⡆",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⢏⡏⣻⡿⢤⣴⡏⠀⣜⣿⣟⣿⠏⣠⢗⢯⠃⡜⡇⠀⠈⡿⣅⡀⠀⠉⡏⢑⢗⣟⠊⠁⢸⢧⠀⢀⠎⡇⠀⢠⢪⠇⠀⠉⣚⣽⢥⠤⠤⡗⠒⣒⠗⡯⣜⠀⠘⡄⢫⡧⣣⠈⢿⣿⣿⡜⣧⠙⣿⣦⠼⣾⠋⣯⣿⣧",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⢸⣷⣷⢇⣰⡿⡈⣽⣻⣿⣾⣟⣩⡃⣸⠃⡜⠀⡇⠀⡜⠀⢣⣈⣢⣄⣗⠕⠁⢹⢣⡀⢸⠈⡆⡜⠀⣇⠔⠁⡎⡠⠔⣪⠋⠀⠑⢕⢴⠥⠊⠀⢀⠜⡄⠉⠒⠼⣴⢣⠙⣧⣈⣿⣿⣿⣻⠉⠸⣇⠀⣿⡷⣺⣿⣿⠶⠤⠤⣤⣤⣤⣤⣄⣀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣾⣧⢻⢸⢹⡇⡿⣇⣿⣿⣿⠃⣼⠉⣟⢾⢶⣶⣻⡝⠉⠉⠉⢇⢠⡳⠹⡦⣀⠈⡆⠙⣼⣤⣾⣶⣭⣯⣵⣾⢯⣄⣜⠥⠤⢒⡲⠟⠹⣳⠒⢤⠮⣀⣘⣄⣠⣤⢾⠟⣏⠙⣤⢻⣿⣿⣿⡇⢰⣿⡄⣿⣳⣼⣿⣿⣯⡭⣉⢀⠇⠈⢺⡈⣉⡿⣷⣦⣄⡀",
        "⠀⠀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣿⣇⣏⣹⣮⣻⣼⣳⣣⣿⣿⣿⣿⠀⣿⣰⠋⣇⠀⠀⣹⢻⠓⠲⠤⣼⣷⠁⠀⠘⡌⠓⢵⣴⣻⣷⣻⣯⡟⣿⢻⡿⣿⣿⣳⣤⣔⣋⣀⣀⣀⣱⣣⡮⠴⠒⠛⢹⡷⡁⠀⡇⢹⣆⡏⢸⣿⣿⣿⣷⣸⢻⡵⡏⣾⢿⣏⣿⣏⠉⠉⢹⣯⠟⠛⡿⣿⣒⣗⡫⣷⢟⣶⣄⡀",
        "⢠⣾⣿⢯⡽⢿⣿⣯⣽⡶⢿⣽⣿⣾⣿⣿⠿⠿⢿⣿⣿⣿⣿⠿⣿⡿⢿⣿⡯⢼⣿⣿⣿⡇⢸⠜⣿⢰⠉⢆⠀⡇⣀⠵⢖⠉⢸⡏⠫⣓⠢⠬⣦⣟⣿⡿⢿⣿⡟⡟⡟⡟⣷⣿⣿⣻⣽⣧⡠⠤⠒⠋⠉⣿⠑⠤⣀⠔⠁⡇⠈⢦⢱⢸⡜⣼⠀⣿⣿⣿⣿⡹⣸⣇⣧⡇⣿⣿⢺⣿⠔⠊⡽⠀⠣⡀⢧⡿⡀⡹⡟⢾⡶⢍⣿⣙⣿⣿⣿⣶⣶⣶⣶⣶⣦⣤⣀⡀",
        "⣾⣿⠔⢱⣇⡞⣿⣿⣟⣷⣜⡟⠛⠬⣿⣲⠽⢞⡷⣟⣿⣿⡿⢭⢿⠛⢵⣏⣇⡾⣿⣿⣿⡳⡝⠀⡏⡜⣀⠬⢶⠋⠀⠀⠀⠣⣿⠀⠀⠀⠑⢄⡜⣿⣿⣛⣿⠭⣪⣺⣿⣪⡪⢽⡿⢯⣿⡟⡌⠑⠒⠤⣀⢸⣇⠔⠉⠒⢄⣳⠀⠀⠙⣼⡇⠹⡄⢻⣿⣿⡏⡇⠹⣿⣏⣷⢿⠺⡏⣿⡦⢄⡇⠀⠀⠑⣼⡠⢫⠇⢇⠇⣿⢏⡰⣧⣿⣿⢟⢉⡩⠭⠛⢻⠿⣻⡿⣻⣆",
        "⣯⣿⢫⣿⡉⣟⣿⣟⡿⣿⢉⣟⢿⡻⣿⣛⠭⣉⣭⣿⣿⡟⣯⢽⣹⡭⢽⣯⡏⡏⣿⣿⣿⡉⢯⠉⣿⢏⠉⠉⢹⡛⢍⡉⢉⠝⣿⠙⠫⢍⣉⠉⢏⣿⡿⣭⣿⠭⡻⣻⣿⡻⡫⢽⣟⣻⣿⣏⠟⢍⠉⠉⠉⢹⡯⡉⠉⠉⠉⣹⡭⠝⢋⠏⡏⢉⢯⣻⣿⣿⣯⡏⠉⣿⡟⢻⡽⢉⣯⣿⡏⠉⡟⢍⡩⠝⢻⢏⠙⡯⣙⡟⡯⣉⣻⣿⡽⣿⡟⠉⠉⢉⣩⣽⣛⣯⣟⣽⡟",
        "⠸⣽⣧⣤⣷⣿⣿⣿⣯⠿⡿⢧⣤⣼⣿⣶⣯⣵⣿⣾⣿⣿⣿⣿⣧⣿⣽⣿⣷⢿⣿⣿⣿⡇⢸⢇⢿⢸⠱⡄⠀⡇⢀⠜⠣⢄⢹⡆⠀⣀⡠⠭⠺⣿⢿⣻⣿⣿⣎⣎⣏⣎⣾⣿⣭⣿⣟⡿⡦⠤⣑⡢⡀⣾⠁⢑⣤⠔⠊⡇⠘⡄⢸⢸⣇⢼⠁⣿⣿⣿⣼⠀⣸⣧⡇⣿⡏⢹⣿⣿⡠⠔⢯⠁⠑⢄⡼⡆⢑⣷⣉⣷⣧⠼⢿⣳⣿⣿⣿⣿⣿⣿⣷⣷⡿⠿⠛⠋",
        "⠀⠈⠛⠛⠚⠛⠛⠓⠚⠛⠛⠛⠚⠛⠛⠛⠛⠛⠛⠛⠛⣿⡟⣿⣻⡟⣿⢿⣺⢻⣿⣿⣿⣷⠈⡞⣾⠀⡇⠈⢢⣷⢁⣀⣠⠤⡿⣻⠛⠓⠒⠒⣒⠿⢿⣻⣿⣧⣿⣹⣏⡿⣿⣻⣿⣻⢿⢅⡈⢆⠀⠉⢹⣿⠮⢥⣀⣱⣰⠃⠀⠘⡇⣸⢻⡎⢰⣿⣿⣿⡿⣰⣹⢿⡷⣿⡡⡞⣿⡟⠑⢄⣸⠀⣀⣀⣷⢷⣷⣋⣫⣽⣿⣿⠿⠛⠉⠉⠉",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣏⢹⣸⡇⢿⣿⠀⣿⣿⣿⣿⡸⢇⠘⣇⣵⡴⠷⠛⢟⠓⠢⡼⢄⣱⣣⢀⣤⣚⡡⠤⢔⠟⠿⣽⣿⠿⡿⢿⣿⡷⢿⢇⠀⡇⠙⠪⣦⢠⡳⠙⡄⠀⠀⢉⢿⣳⣶⣴⣝⡇⢸⠇⣸⣿⣿⡟⣧⡇⣿⢸⣿⡿⣿⣷⣿⡟⢋⣉⠽⡝⠛⢫⠯⢍⣦⣾⡿⠟⠋",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿⠀⣿⠃⠘⣧⠀⢹⣻⣿⣿⡗⠺⣏⠹⣹⢦⢄⡀⠈⢆⠜⠀⠀⡠⢽⢝⢅⠀⠀⡰⣋⠤⠊⡏⢀⠝⡏⢁⠎⡆⢸⠈⠳⣸⠀⢀⢜⡗⠷⡒⠻⡉⢉⠏⠉⡇⢀⠎⣸⠋⡻⢒⣿⣿⣿⣳⠏⢣⣿⠛⣿⣷⣿⣺⣿⣭⠭⠶⠶⠵⠴⠾⠞⠛⠉⠁",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⡿⡆⣹⣧⠴⣿⣎⠹⣏⢿⣿⣿⡄⠹⡣⡷⡈⢆⠈⢑⠮⣆⣔⣊⣀⡧⠤⠵⢵⢞⡋⠀⠀⡼⡰⠁⠀⡇⡜⠀⠸⣼⠀⠀⣙⣗⣕⠁⡇⠀⠈⠒⢧⡎⠀⠀⣇⠎⡰⣝⡴⠋⣽⣟⣾⣟⠏⠉⣾⠷⢤⣷⡃⡟⣼⡿",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠹⡍⣞⡦⣸⢻⡕⢏⢞⣿⣿⣿⡄⠱⡏⢷⡈⢆⡮⠚⠁⠑⢄⢰⠁⠀⡠⢻⠉⢚⣵⣶⣯⣤⣀⣀⣟⣀⣠⣤⣿⡲⠝⢺⠁⠣⡉⢹⠑⢒⠖⢽⠽⢦⣄⢿⡲⢩⡳⠁⣼⣿⣿⢯⠏⢀⣼⡇⢀⣼⠏⣹⣻⣻⡇",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣧⠹⣻⢞⡌⣧⢻⢼⡫⣿⣿⣿⡽⣴⠱⡠⠗⢟⡷⢤⠤⠤⢄⣝⣄⡼⠤⠚⡏⠁⢀⡏⢆⢀⠭⠋⡟⢍⠁⡔⠉⣏⠒⡧⣀⠀⠘⢼⠔⠁⠀⡇⢀⠔⢉⡝⠧⡼⠁⣼⣿⣿⣿⢿⢔⢽⢻⡠⣞⠏⣰⣿⣳⡟",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣷⡹⡟⢞⣽⠑⢯⢇⠈⢯⣿⣾⢝⣍⠈⠳⡢⣳⢄⠑⢄⢀⡷⠋⠉⠒⠤⣇⠀⣼⣀⠜⢇⠀⠀⡇⢠⠛⢄⠀⢸⣼⢀⡠⠕⢪⢏⡳⡀⢀⡗⢁⣔⣝⡠⠮⢜⢟⢿⣿⣟⡯⣇⢮⡣⡏⣜⡞⡴⣹⣳⡟",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣷⣻⢵⡈⢾⡆⠀⢻⣆⡼⢫⡫⣷⣽⣷⣄⠈⠺⡑⠳⢄⣟⣥⣤⠤⠤⠤⢷⣻⠟⠲⠤⠬⢦⣀⣷⣡⠤⠤⠵⣶⢿⡁⠀⢠⠃⢀⡨⠝⢿⠝⢩⡪⠊⢀⣴⣷⣵⣫⡯⠻⣴⢯⠗⢹⣜⣮⡻⢱⣿⡟",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣷⣫⡷⡈⣿⣴⡞⠱⣝⢖⢯⡻⣿⣿⣿⣷⢄⢏⡢⣏⣉⡚⡤⣉⠒⠤⣸⣟⠀⢀⡠⠔⠊⢇⡇⠀⢀⡤⠊⠀⢙⣧⣠⣗⢊⡡⡴⠞⢉⡣⡋⢀⣴⣿⣿⣿⣿⠋⢀⢴⠗⠙⢶⣟⡞⡾⢡⣿⡟",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣷⣳⡟⢥⡳⣝⢕⠳⣑⢕⢽⡙⠻⣿⡳⣕⣽⣧⣄⡉⠚⠽⣕⠛⠲⡾⠵⢯⣉⣑⣒⡒⠬⣧⣔⣓⣒⣉⣉⣥⠽⡗⢊⣭⡫⠝⠋⣑⡶⣗⣝⡵⡿⡿⣫⢗⢞⡵⠃⣠⢴⠟⢁⣝⣧⣿⡟",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠪⡻⣦⡙⢕⡵⣕⢜⡍⠳⢕⣄⠈⣫⣿⢿⣿⣿⣿⣷⣦⣌⣏⢺⠓⠶⠶⠾⣛⣍⣉⠉⡏⣉⣭⣕⡺⠭⣤⡔⢻⠋⣁⣤⣶⣿⣷⣿⡾⣿⣯⡚⣹⢕⢕⡱⢓⢝⡵⢁⣴⡻⣫⡮⠊",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣿⡢⣝⢎⠓⢝⣆⠀⢙⣝⡷⣬⠯⢟⣿⣟⡿⣿⣽⣻⣿⣿⣶⣶⣶⣤⣤⣵⣫⣏⣡⣤⣤⣴⣶⣶⣾⣿⣿⣿⣿⣿⢿⡿⡿⠋⢁⣠⣽⣕⠋⣝⢔⢕⢯⠞⡵⣫⡾⠋",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣮⡻⣳⣄⠙⢷⢿⣄⣙⡵⡫⢖⣕⣄⠉⠛⠽⣿⣻⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣻⣯⡵⢮⠕⣛⣞⢯⠗⠉⠀⣘⣿⠵⣫⠗⣡⣾⡾⠋",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⡮⣻⡵⠯⣽⣺⡵⣪⢍⡳⡚⠻⠷⢦⣤⣃⡩⠿⠯⢭⣛⠛⠓⠺⠯⡯⠭⠿⣛⣿⣛⣉⣉⠁⣀⣣⣴⣷⣛⡭⠶⣗⢶⡺⠟⢋⡩⢷⣥⡾⡫⠋",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠺⢿⣷⣦⣍⣚⠯⣝⠚⠽⣶⣄⣀⡟⠙⢻⣟⣛⣛⣶⣿⡶⢦⣤⣧⣶⣿⠶⠶⣞⠿⠟⠛⠉⠙⣏⣠⢮⣲⣝⣾⡥⣔⣞⢯⣻⣽⠾⠊",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠻⢿⣿⣲⢿⣶⣤⣉⡿⠻⢽⣷⠶⣒⣫⣶⣤⣤⣄⣀⣇⣤⠶⠯⢭⣭⣛⣻⠿⠶⠟⠻⣎⡭⠟⣊⣵⣾⣽⠾⠛⠉",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠻⢾⣽⣺⣯⣝⣛⣛⣻⣿⠧⢤⣉⣉⣉⠓⡗⣚⣻⣿⣳⠾⣧⣴⣶⣶⣾⣯⣷⣶⣿⠿⠛⠉",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠛⠛⠛⠿⠿⠿⢿⣿⣿⣿⣯⣯⣭⣶⣶⣾⠿⠿⠿⠟⠛⠛⠋⠉⠉",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠉",
    ]);
}
