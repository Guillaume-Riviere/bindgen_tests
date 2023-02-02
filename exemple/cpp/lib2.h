
struct Vector2{
    public:
        double x;
        double y;
    Vector2(double ix = 0, double iy = 0);
        double distanceTo(Vector2 pos);
        void vectorMovement(double plusx, double plusy);
        Vector2 midpoint(Vector2 pos);
        double percentDistance(Vector2 pos, double percentOfDistance = 100);
};
