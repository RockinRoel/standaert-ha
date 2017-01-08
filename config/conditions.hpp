#ifndef CONDITIONS_HPP
#define CONDITIONS_HPP

#include "Light.hpp"
#include "LightGroup.hpp"

class Condition {
public:
  virtual bool isMet() const = 0;
};

using ConditionPtr = std::unique_ptr<Condition>;

class NotCondition : public Condition {
public:
  NotCondition(ConditionPtr condition)
    : condition_{std::move(condition)}
  { }

  const Condition &condition() const { return *condition_; }

  virtual bool isMet() const override
  {
    return !condition().isMet();
  }

private:
  ConditionPtr condition_;
};

class BinaryCondition : public Condition {
public:
  BinaryCondition(ConditionPtr leftCondition,
                  ConditionPtr rightCondition);

  const Condition &leftCondition() const { return *leftCondition_; }
  const Condition &rightCondition() const { return *rightCondition_; }

private:
  ConditionPtr leftCondition_;
  ConditionPtr rightCondition_;
};

class AndCondition : public BinaryCondition {
public:
  AndCondition(ConditionPtr leftCondition,
               ConditionPtr rightCondition)
    : BinaryCondition{std::move(leftCondition), std::move(rightCondition)}
  { }

  virtual bool isMet() const override
  {
    return leftCondition().isMet() && rightCondition().isMet();
  }
};

class OrCondition : public BinaryCondition {
public:
  OrCondition(ConditionPtr leftCondition,
              ConditionPtr rightCondition)
    : BinaryCondition{std::move(leftCondition), std::move(rightCondition)}
  { }

  virtual bool isMet() const override
  {
    return leftCondition().isMet() || rightCondition().isMet();
  }
};

class XorCondition : public BinaryCondition {
public:
  XorCondition(ConditionPtr leftCondition,
               ConditionPtr rightCondition)
    : BinaryCondition{std::move(leftCondition), std::move(rightCondition)}
  { }

  virtual bool isMet() const override
  {
    return leftCondition().isMet() != rightCondition().isMet();
  }
};

class LightStateCondition : public Condition {
public:
  LightStateCondition(const Light *light, bool state)
    : light_{light},
      state_{state}
  { }

  virtual bool isMet() const override
  {
    return light_->isOn() == state_;
  }

private:
  const Light *light_;
  bool state_;
};

class LightGroupStateCondition : public Condition {
public:
  LightGroupStateCondition(const LightGroup *lightGroup, LightGroup::State state)
    : lightGroup_{lightGroup},
      state_{state}
  { }

  virtual bool isMet() const override
  {
    return lightGroup_->state() == state_;
  }

private:
  const LightGroup *lightGroup_;
  LightGroup::State state_;
};

#endif // CONDITIONS_HPP
