#include "Light.hpp"

Light::Light()
  : isOn_{false}
{ }

void Light::fromJson(const Wt::Json::Value &json)
{
  throw stub_exception{};
}

Wt::Json::Value Light::toJson() const
{
  throw stub_exception{};
}
