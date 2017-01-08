#ifndef LIGHT_GROUP_HPP
#define LIGHT_GROUP_HPP

#include "model.hpp"

class Light;

class LightGroup : public HAObject {
public:
  enum class State {
    on,
    off,
    inconsistent
  };

  State state() const;

  virtual void fromJson(const Wt::Json::Value &json) override;

  virtual Wt::Json::Value toJson() const override;

  std::vector<Light*> lights;
};

#endif // LIGHT_GROUP_HPP
