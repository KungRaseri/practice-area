


# Database
## Generate Database Entities
```diesel_ext -m -I "sql_types::*" -I "serde::{Serialize, Deserialize}" -t -d "Serialize, Deserialize, Queryable, Clone, Copy" > src/entities.rs```