use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Clone)]
pub struct Matrix<T> {
    pub width: usize,
    pub height: usize,
    pub(crate) data: Vec<T>,
}

#[allow(dead_code)]
pub const MATRIX_NEIGHBOURS_4: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

#[allow(dead_code)]
pub const MATRIX_NEIGHBOURS_8: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (0, -1),
    (-1, 0),
    (-1, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
];

impl<T> Matrix<T> {
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.width * self.height
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = (&T, (usize, usize))> {
        self.data.iter().enumerate().map(|(index, value)| {
            let x = index % self.width;
            let y = index / self.width;

            (value, (x, y))
        })
    }

    #[allow(dead_code)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut T, (usize, usize))> {
        self.data.iter_mut().enumerate().map(|(index, value)| {
            let x = index % self.width;
            let y = index / self.width;

            (value, (x, y))
        })
    }

    #[allow(dead_code)]
    pub fn iter_with_self(&self) -> impl Iterator<Item = (&T, (usize, usize), &Matrix<T>)> {
        self.data.iter().enumerate().map(move |(index, value)| {
            let x = index % self.width;
            let y = index / self.width;

            (value, (x, y), self)
        })
    }
}

impl<T: Debug + Default> Debug for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix({}x{})", self.width, self.height)?;

        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Some(x) => write!(f, "{:?}", x)?,
                    None => write!(f, " ")?,
                };
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T: Display + Default> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Some(x) => write!(f, "{}", x)?,
                    None => write!(f, " ")?,
                };
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> Matrix<T> {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Matrix<T>
    where
        T: Default,
    {
        let mut data = Vec::new();
        data.resize_with(width * height, T::default);

        Self {
            width,
            height,
            data,
        }
    }

    #[allow(dead_code)]
    pub fn from_iter<'a>(iter: impl Iterator<Item = &'a T>, width: usize) -> Matrix<T>
    where
        T: 'a + Default + Copy,
    {
        let mut data: Vec<T> = iter.copied().collect();
        let mut height = data.len() / width;
        height += if width * height < data.len() { 1 } else { 0 };

        data.resize_with(width * height, T::default);

        Self {
            width,
            height,
            data,
        }
    }

    fn get_index<P>(&self, x: P, y: P) -> Option<usize>
    where
        P: TryInto<i32>,
    {
        let x = x.try_into().ok().unwrap();
        let y = y.try_into().ok().unwrap();
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            return None;
        }
        let position = x + y * self.width as i32;
        Some(position as usize)
    }

    pub fn get<P>(&self, x: P, y: P) -> Option<&T>
    where
        P: TryInto<i32>,
    {
        self.get_index(x, y).and_then(|index| self.data.get(index))
    }

    #[allow(dead_code)]
    pub fn get_mut<P>(&mut self, x: P, y: P) -> Option<&mut T>
    where
        P: TryInto<i32>,
    {
        self.get_index(x, y)
            .and_then(|index| self.data.get_mut(index))
    }

    #[allow(dead_code)]
    pub fn set<P>(&mut self, x: P, y: P, value: T)
    where
        P: TryInto<i32>,
    {
        if let Some(index) = self.get_index(x, y) {
            self.data[index] = value;
        }
    }

    #[allow(dead_code)]
    pub fn neighbours4<P>(&self, x: P, y: P) -> Vec<&T>
    where
        P: TryInto<i32>,
    {
        let x = x.try_into().ok().unwrap();
        let y = y.try_into().ok().unwrap();

        MATRIX_NEIGHBOURS_4
            .iter()
            .filter_map(|(dx, dy)| self.get(x + dx, y + dy))
            .collect()
    }

    #[allow(dead_code)]
    pub fn neighbours8<P>(&self, x: P, y: P) -> Vec<&T>
    where
        P: TryInto<i32>,
    {
        let x = x.try_into().ok().unwrap();
        let y = y.try_into().ok().unwrap();

        MATRIX_NEIGHBOURS_8
            .iter()
            .filter_map(|(dx, dy)| self.get(x + dx, y + dy))
            .collect()
    }

    #[allow(dead_code)]
    pub fn neighbours_iter<'a, P>(
        &'a self,
        offsets: &'a [(i32, i32)],
        x: P,
        y: P,
    ) -> impl Iterator<Item = (&'a T, (i32, i32))> + 'a
    where
        P: TryInto<i32>,
    {
        let x = x.try_into().ok().unwrap();
        let y = y.try_into().ok().unwrap();

        offsets.iter().filter_map(move |(dx, dy)| {
            self.get(x + dx, y + dy)
                .map(|value| (value, (x + dx, y + dy)))
        })
    }

    #[allow(dead_code)]
    pub fn neighbours8_iter<P>(&self, x: P, y: P) -> impl Iterator<Item = (&T, (i32, i32))>
    where
        P: TryInto<i32>,
    {
        self.neighbours_iter(&MATRIX_NEIGHBOURS_8, x, y)
    }

    #[allow(dead_code)]
    pub fn neighbours4_iter<P>(&self, x: P, y: P) -> impl Iterator<Item = (&T, (i32, i32))>
    where
        P: TryInto<i32>,
    {
        self.neighbours_iter(&MATRIX_NEIGHBOURS_4, x, y)
    }

    #[allow(dead_code)]
    pub fn render_to_string<F>(&self, renderer: F) -> String
    where
        F: Fn(Option<&T>) -> String,
    {
        let rows = (0..self.height)
            .map(|y| (0..self.width).map(|x| renderer(self.get(x, y))).collect())
            .collect::<Vec<Vec<_>>>();

        rows.iter()
            .map(|row| row.join(""))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl<T: Default + FromStr> Matrix<T> {
    #[allow(dead_code)]
    pub fn from(s: &str) -> Option<Matrix<T>> {
        let lines = s.lines().map(|l| l.to_string()).collect::<Vec<String>>();
        let width = match lines.get(0) {
            None => return None,
            Some(s) => s.len(),
        };
        let height = lines.len();

        let data = lines
            .iter()
            .flat_map(|line| {
                line.chars()
                    .map(|ch| ch.to_string().parse::<T>().unwrap_or_default())
                    .collect::<Vec<T>>()
            })
            .collect::<Vec<T>>();

        Some(Self {
            width,
            height,
            data,
        })
    }

    #[allow(dead_code)]
    pub fn from_separated(s: &str, pat: &str) -> Option<Matrix<T>> {
        let lines = s.lines().map(|l| l.to_string()).collect::<Vec<String>>();
        let width = match lines.get(0) {
            None => return None,
            Some(s) => s.len(),
        };
        let height = lines.len();
        let mut data: Vec<T> = vec![];

        for line in lines {
            data.extend(line.split(pat).map(|s| s.parse::<T>().unwrap_or_default()));
        }

        Some(Self {
            width,
            height,
            data,
        })
    }
}
