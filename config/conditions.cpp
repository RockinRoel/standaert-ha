#include "conditions.hpp"

BinaryCondition::BinaryCondition(ConditionPtr leftCondition,
                                 ConditionPtr rightCondition)
  : leftCondition_{std::move(leftCondition)},
    rightCondition_{std::move(rightCondition)}
{ }
