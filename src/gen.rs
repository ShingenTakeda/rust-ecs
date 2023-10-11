#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GenData{
    pub pos: usize,
    pub gen: u64,
}

pub struct EntityActive{
    active: bool,
    gen: u64
}

pub struct GenManager{
    items: Vec<EntityActive>,
    drops: Vec<usize>
}

impl GenManager{
    pub fn new()->Self{
        GenManager{
            items: Vec::new(),
            drops: Vec::new()
        }
    }

    pub fn next(&mut self)->GenData{
        if let Some(loc) = self.drops.pop(){
            let ea = &mut self.items[loc];
            ea.active = true;
            ea.gen += 1;
            return GenData{
                pos:loc,
                gen:ea.gen,
            }
        }
        self.items.push(EntityActive{active: true, gen: 0});
        return GenData{
            gen: 0,
            pos: self.items.len() - 1,
        };
    }

    pub fn drop(&mut self, g:GenData){
            if let Some(ea) = self.items.get_mut(g.pos) {
                if ea.active && ea.gen == g.gen {
                    ea.active = false;

                    self.drops.push(g.pos)
                }
            }
        }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_itesm_drop() {
        let mut gm = GenManager::new();
        
        let g = gm.next();
        let a = gm.next();
        let b = gm.next();

        assert_eq!(g, GenData{gen:0, pos:0});
        assert_eq!(a, GenData{gen:0, pos:1});   
        assert_eq!(b, GenData{gen:0, pos:2});

        gm.drop(a);

        let d = gm.next();

        assert_eq!(d, GenData{gen:1, pos: 1})
    }
}
