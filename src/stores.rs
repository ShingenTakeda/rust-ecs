use crate::gen::GenData;

pub trait EcsStore<T>{
    fn add(&mut self, f:GenData, t:T);
    fn get(&self, g:GenData)->Option<&T>;
    fn get_mut(&mut self, g:GenData)->Option<&mut T>;
    fn drop(&mut self, g:GenData);

    fn for_each<F: FnMut(GenData, &T)>(&self, f:F);
    fn for_each_mut<F: FnMut(GenData, &mut T)>(&mut self, f:F);
}

pub struct VecStore<T>{
    items: Vec<Option<(u64, T)>>
}

impl <T> VecStore<T>{
    pub fn new()->Self{
        VecStore { items: Vec::new() }
    }
}

impl <T> EcsStore<T> for VecStore<T>{
    fn add(&mut self, gen: GenData, t: T){
        while g.pos >= self.items.len(){
            self.items.push(None)
        }

        self.items[g.pos] = Some((g.gen, t));
    }

    fn get(&self, g:GenData)->Option<&T>{
        if let Some(Some((ig, d))) = self.items.get(g.pos) {
            if *ig == g.gen{
                return Some(d)
            }
        }
        None
    }

    fn get_mut(&mut self, g:GenData)->Option<&mut T>{
        if let Some(Some((ig, d))) = self.items.get_mut(g.pos) {
            if *ig == g.gen{
                return Some(d)
            }
        }
        None
    }

    fn drop(&mut self, g:GenData){
        if let Some(Some((ig, _))) = self.items.get(g.pos) {
            if *ig == g.gen{
                self.items[g.pos] = None;
            }
        }
    }

    fn for_each<F: FnMut(GenData, &T)>(&self, mut f:F){
        for(n,x) in self.items.iter_mut().enumerate(){
            if let Some((g,d)) = x {
                f(GenData{gen: *g, pos:n}, d);
            }
        }
    } 

    fn for_each_mut<F: FnMut(GenData, &mut T)>(&mut self, mut f:F){
        for(n,x) in self.items.iter_mut().enumerate(){
            if let Some((g,d)) = x {
                f(GenData{gen: *g, pos:n}, d);
            }
        }
    }
}
