pub struct SegTree {
    t: Vec<i32>,
    a: Vec<i32>,
}

impl SegTree {
    fn build(&mut self, v: usize, l: usize, r: usize) {
        if r - l == 1 {
            self.t[v] = self.a[l];
            return;
        }

        let m = (l + r) / 2;
        self.build(2 * v + 1, l, m);
        self.build(2 * v + 2, m, r);
        self.t[v] = self.t[2 * v + 1] + self.t[2 * v + 2];
    }

    pub fn new(arr: &[i32]) -> Self {
        let n = arr.len();

        let tmp = vec![0; 8 * n];
        let mut tree = SegTree {
            t: tmp,
            a: arr.to_vec(),
        };

        tree.build(0, 0, arr.len());
        tree
    }

    pub fn ask(&mut self, v: usize, l: usize, r: usize, ql: usize, qr: usize) -> i32 {
        if l >= qr || r <= ql {
            0
        } else if l >= ql && r <= qr {
            self.t[v]
        } else {
            let m = (l + r) / 2;
            self.ask(2 * v + 1, l, m, ql, qr) + self.ask(2 * v + 2, m, r, ql, qr)
        }
    }

    pub fn upd(&mut self, v: usize, l: usize, r: usize, pos: usize, val: i32) {
        if l == r - 1 {
            self.t[v] = val;
            return;
        }

        let m = (l + r) / 2;
        if pos < m {
            self.upd(2 * v + 1, l, m, pos, val);
        } else {
            self.upd(2 * v + 2, m, r, pos, val);
        }
        self.t[v] = self.t[2 * v + 1] + self.t[2 * v + 2];
    }
}

#[cfg(test)]
mod test {
    use super::SegTree;

    #[test]
    fn non_empty_arr_without_changes() {
        let arr = vec![1, 2, 3, 4, 5];
        let mut tree: SegTree = SegTree::new(&arr);
        assert_eq!(tree.ask(0, 0, arr.len(), 0, arr.len()), 15);
        assert_eq!(tree.ask(0, 0, arr.len(), 0, arr.len() - 1), 10);
        assert_eq!(tree.ask(0, 0, arr.len(), 1, arr.len()), 14);
        assert_eq!(tree.ask(0, 0, arr.len(), 1, arr.len() - 1), 9);
    }

    #[test]
    fn non_empty_arr_with_changes() {
        let arr = [1, 2, 3, 4, 5];
        let mut tree: SegTree = SegTree::new(&arr);
        assert_eq!(tree.ask(0, 0, arr.len(), 0, arr.len()), 15);
        tree.upd(0, 0, arr.len(), 0, 100);
        assert_eq!(tree.ask(0, 0, arr.len(), 0, arr.len()), 114);
    }
}
