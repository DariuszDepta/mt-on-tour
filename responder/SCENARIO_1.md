# Scenario 1

```yaml
dmn:
  namespace: "https://bank.com/dmn"
  id: "loan_v1"
  decisions: [approval, income_check]
  inputs: [income, credit_score]
```


| Income Valid | Credit Score | Outcome  |
|--------------|--------------|----------|
| **input**    | IN           | OUT      |
| true         | \>=700       | Approved |
| true         | [650..699]   | Pending  |
| false        | -            | Denied   |

