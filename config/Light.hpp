#ifndef LIGHT_HPP
#define LIGHT_HPP

#include "model.hpp"

class Light : public HAObject {
public:
  Light();

  bool isOn() const { return isOn_; }

  virtual void fromJson(const Wt::Json::Value &json) override;

  virtual Wt::Json::Value toJson() const override;

private:
  bool isOn_;
};

#endif // LIGHT_HPP
