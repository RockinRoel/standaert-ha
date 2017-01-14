#include <iostream>

#include "config/model.hpp"
#include "config/conditions.hpp"

#include <Wt/WApplication.h>
#include <Wt/WServer.h>

int main(int argc, char *argv[])
{
  return Wt::WRun(argc, argv, [](const Wt::WEnvironment &env) {
    return Wt::cpp14::make_unique<Wt::WApplication>(env);
  });
}
