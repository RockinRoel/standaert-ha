#include "LightGroup.hpp"

#include "Light.hpp"

LightGroup::State LightGroup::state() const
{
  std::size_t nbOn = 0;
  for (const Light *light : lights) {
    if (light->isOn())
      ++nbOn;
  }
  if (nbOn == 0)
    return State::off;
  else if(lights.size() == nbOn)
    return State::on;
  else
    return State::inconsistent;
}

void LightGroup::fromJson(const Wt::Json::Value &json)
{
  throw stub_exception{};
}

Wt::Json::Value LightGroup::toJson() const
{
  throw stub_exception{};
}
