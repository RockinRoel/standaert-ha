#ifndef MODEL_HPP
#define MODEL_HPP

#include <exception>
#include <memory>
#include <string>
#include <vector>

#include <Wt/WString.h>

#include <Wt/Json/Object.h>
#include <Wt/Json/Value.h>

class Configuration; // hardware (i2cbus, mcp23017,...), lights, lightgroups, buttons, buttongroups, rules, timers
class HAObject; // id
class I2CBus; // ???
class MCP23017; // bus, address
class Light; // output
class LightGroup; // lights
class Button; // input
class ButtonGroup; // buttons
class Rule; // source event, filter, actions
class Timer; // duration

// Event (or Signal)
// Filters?
// Conditions? BoolCondition

class stub_exception : public std::exception {
};

class Identifier {
public:
  Identifier() { }

  Identifier(std::string id,
             std::string type)
    : id_{id},
      type_{type}
  { }

  const std::string &id() const { return id_; }
  const std::string &type() const { return type_; }

  Wt::Json::Value toJson() const
  {
    Wt::Json::Value result;
    Wt::Json::Object &o = result;
    o["id"] = Wt::utf8(id());
    o["type"] = Wt::utf8(type());
    return result;
  }

  bool isValid() const {
    return !id().empty() && !type().empty();
  }

private:
  const std::string id_;
  const std::string type_;
};

class HAObject {
public:
  virtual void fromJson(const Wt::Json::Value &json) = 0;
  virtual Wt::Json::Value toJson() const = 0;

  Wt::Json::Value toJsonRef() const
  {
    return id.toJson();
  }

  Identifier id;
};

class I2CBus : public HAObject {
public:
  virtual void fromJson(const Wt::Json::Value &json) override
  {
  }

  virtual Wt::Json::Value toJson() const override
  {
    throw stub_exception{};
  }
};

class MCP23017 : public HAObject {
public:
  virtual void fromJson(const Wt::Json::Value &json) override
  {
  }

  virtual Wt::Json::Value toJson() const override
  {
    throw stub_exception{};
  }

  I2CBus *bus;
  int address; // 3 bits
};

class Button : public HAObject {
public:
  virtual void fromJson(const Wt::Json::Value &json) override
  {
  }

  virtual Wt::Json::Value toJson() const override
  {
    throw stub_exception{};
  }
};

class ButtonGroup : public HAObject {
public:
  virtual void fromJson(const Wt::Json::Value &json) override
  {
  }

  virtual Wt::Json::Value toJson() const override
  {
    throw stub_exception{};
  }

  std::vector<Button*> buttons;
};

class Rule : public HAObject {
public:
  class Action {
    // conditions, target
  };

  virtual void fromJson(const Wt::Json::Value &json) override
  {
  }

  virtual Wt::Json::Value toJson() const override
  {
    throw stub_exception{};
  }

  std::vector<Action> actions;
};

class Timer : public HAObject {
public:
  virtual void fromJson(const Wt::Json::Value &json) override
  {
  }

  virtual Wt::Json::Value toJson() const override
  {
    throw stub_exception{};
  }
};

#endif // MODEL_HPP
