pub struct Section{
    section: String,
    bullets: Vec<Bullet>,
}

impl HasTask for Section{
    fn is_open(&self) => bool {
        for bullet in self.bullets {
            if bullet.is_open() {
                return True;
            }
        }
        return False;
    }
}