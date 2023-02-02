struct Vector2{
    x: f64,
    y: f64,
}

impl Vector2 {
    fn Vector2(ix: f64, iy: f64) -> Vector2{
        return Vector2{x : ix, y : iy};
    }
    fn distanceTo(&self, pos : Vector2) -> f64
    {
        return (pos.y - self.y) * (pos.y - self.y) + (pos.x - self.x) * (pos.x - self.x).sqrt();
    }
    fn vectorMovement(&mut self, plusx : f64, plusy : f64)
    {
        self.x += plusx;
        self.y += plusy;
    }
    fn midpoint(&self, pos : Vector2) -> Vector2
    {
        let mx = (self.x + pos.x) / 2.0;
        let my = (self.y + pos.y) / 2.0;
        let mid = Vector2{ x : mx, y : my};
        return mid;
    }
    fn percentDistance(&self, pos : Vector2, percentOfDistance : f64) -> f64
    {
        self.distanceTo(pos) / (100.0 / percentOfDistance)
    }
}