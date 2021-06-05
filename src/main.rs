use image::{io::Reader, Rgba, RgbaImage};
use std::fmt;
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;

const DARK: [u8; 3] = [0x36, 0x39, 0x3E];
const LIGHT: [u8; 3] = [0xFF, 0xFF, 0xFF];

const STDOUT_FAIL: &str = "Failed to write stdout.";
const STDIN_FAIL: &str = "Failed to read stdin.";
const PATH_FAIL: &str = "Invalid path.";

fn main()
{
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    write!(stdout, "Path to dark mode image: ").expect(STDOUT_FAIL);
    stdout.flush().expect(STDOUT_FAIL);
    stdin.read_line(&mut input).expect(STDIN_FAIL);
    input = input.trim().to_owned();
    let dark_path = Path::new(&input).to_owned();
    if !dark_path.is_file()
    {
        eprintln!("{}", PATH_FAIL);
        exit(1);
    }

    input.clear();

    write!(stdout, "Path to light mode image: ").expect(STDOUT_FAIL);
    stdout.flush().expect(STDOUT_FAIL);
    stdin.read_line(&mut input).expect(STDIN_FAIL);
    input = input.trim().to_owned();
    let light_path = Path::new(&input).to_owned();
    if !light_path.is_file()
    {
        eprintln!("{}", PATH_FAIL);
        exit(1);
    }

    input.clear();

    write!(stdout, "Path to save to: ").expect(STDOUT_FAIL);
    stdout.flush().expect(STDOUT_FAIL);
    stdin.read_line(&mut input).expect(STDIN_FAIL);
    input = input.trim().to_owned();
    let save_path = Path::new(&input).to_owned();
    let mut save_dir = save_path.clone();
    save_dir.pop();
    if !save_dir.is_dir()
    {
        eprintln!("{}", PATH_FAIL);
        exit(1);
    }

    if let Err(why) = mix(&dark_path, &light_path, &save_path)
    {
        eprintln!("{}", why);
        exit(1);
    }
}

fn mix(dark: &Path, light: &Path, save: &Path) -> Result<(), Box<dyn std::error::Error>>
{
    struct IncompatibleSizesError;
    impl std::error::Error for IncompatibleSizesError {}
    impl fmt::Display for IncompatibleSizesError
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
        {
            <Self as fmt::Debug>::fmt(self, f)
        }
    }
    impl fmt::Debug for IncompatibleSizesError
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
        {
            write!(f, "Image sizes must match.")
        }
    }

    let dark_img = Reader::open(dark)?.decode()?.into_rgba8();
    let light_img = Reader::open(light)?.decode()?.into_rgba8();
    let size = {
        let (dw, dh) = (dark_img.width(), dark_img.height());
        let (lw, lh) = (light_img.width(), light_img.height());
        if dw == lw && dh == lh
        {
            (dw, dh)
        }
        else
        {
            return Err(Box::new(IncompatibleSizesError));
        }
    };
    let mut new_img = RgbaImage::new(size.0, size.1);

    for x in 0..size.0
    {
        for y in 0..size.1
        {
            if (x + y) % 2 == 0
            {
                let a = dark_img.get_pixel(x, y).0[3];
                new_img.put_pixel(x, y, Rgba([LIGHT[0], LIGHT[1], LIGHT[2], a]));
            }
            else
            {
                let a = light_img.get_pixel(x, y).0[3];
                new_img.put_pixel(x, y, Rgba([DARK[0], DARK[1], DARK[2], a]));
            }
        }
    }

    new_img.save(save)?;

    Ok(())
}
