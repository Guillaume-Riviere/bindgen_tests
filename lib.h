class FloatValue {
public:
	FloatValue();
	FloatValue(float value);
	~FloatValue();

	float Get() const;
	void Set(float value);
};

FloatValue operator+(const FloatValue &a, const FloatValue &b);