use sqlparser::ast::Statement;

use super::executors::{insert_executor::InsertExecutor, Executor, create_table_executor::CreateTableExecutor};

//NOTA: MIGRATE THIS TO A PROC MACRO
pub fn parse_statement(statement: &Statement) -> Executor {
    match statement {
        Statement::Insert { .. } => {
                let executor = InsertExecutor::from(statement.clone());
                Executor::from(executor)
        },        
        Statement::CreateTable {..} => {
                let executor = CreateTableExecutor::from(statement.clone());
                Executor::from(executor)
            },
        _ => unimplemented!()
        // Statement::Analyze { table_name, partitions, for_columns, columns, cache_metadata, noscan, compute_statistics } => todo!(),
        // Statement::Truncate { table_name, partitions, table } => todo!(),
        // Statement::Msck { table_name, repair, partition_action } => todo!(),
        // Statement::Query(_) => todo!(),
        // Statement::Directory { overwrite, local, path, file_format, source } => todo!(),
        // Statement::Copy { source, to, target, options, legacy_options, values } => todo!(),
        // Statement::CopyIntoSnowflake { into, from_stage, from_stage_alias, stage_params, from_transformations, files, pattern, file_format, copy_options, validation_mode } => todo!(),
        // Statement::Close { cursor } => todo!(),
        // Statement::Update { table, assignments, from, selection, returning } => todo!(),
        // Statement::Delete { tables, from, using, selection, returning, order_by, limit } => todo!(),
        // Statement::CreateView { or_replace, materialized, name, columns, query, with_options, cluster_by, with_no_schema_binding, if_not_exists, temporary } => todo!(),
        // Statement::CreateVirtualTable { name, if_not_exists, module_name, module_args } => todo!(),
        // Statement::CreateIndex { name, table_name, using, columns, unique, concurrently, if_not_exists, include, nulls_distinct, predicate } => todo!(),
        // Statement::CreateRole { names, if_not_exists, login, inherit, bypassrls, password, superuser, create_db, create_role, replication, connection_limit, valid_until, in_role, in_group, role, user, admin, authorization_owner } => todo!(),
        // Statement::AlterTable { name, if_exists, only, operations } => todo!(),
        // Statement::AlterIndex { name, operation } => todo!(),
        // Statement::AlterView { name, columns, query, with_options } => todo!(),
        // Statement::AlterRole { name, operation } => todo!(),
        // Statement::AttachDatabase { schema_name, database_file_name, database } => todo!(),
        // Statement::Drop { object_type, if_exists, names, cascade, restrict, purge, temporary } => todo!(),
        // Statement::DropFunction { if_exists, func_desc, option } => todo!(),
        // Statement::Declare { name, binary, sensitive, scroll, hold, query } => todo!(),
        // Statement::Fetch { name, direction, into } => todo!(),
        // Statement::Discard { object_type } => todo!(),
        // Statement::SetRole { context_modifier, role_name } => todo!(),
        // Statement::SetVariable { local, hivevar, variable, value } => todo!(),
        // Statement::SetTimeZone { local, value } => todo!(),
        // Statement::SetNames { charset_name, collation_name } => todo!(),
        // Statement::SetNamesDefault {  } => todo!(),
        // Statement::ShowFunctions { filter } => todo!(),
        // Statement::ShowVariable { variable } => todo!(),
        // Statement::ShowVariables { filter, global, session } => todo!(),
        // Statement::ShowCreate { obj_type, obj_name } => todo!(),
        // Statement::ShowColumns { extended, full, table_name, filter } => todo!(),
        // Statement::ShowTables { extended, full, db_name, filter } => todo!(),
        // Statement::ShowCollation { filter } => todo!(),
        // Statement::Use { db_name } => todo!(),
        // Statement::StartTransaction { modes, begin } => todo!(),
        // Statement::SetTransaction { modes, snapshot, session } => todo!(),
        // Statement::Comment { object_type, object_name, comment, if_exists } => todo!(),
        // Statement::Commit { chain } => todo!(),
        // Statement::Rollback { chain, savepoint } => todo!(),
        // Statement::CreateSchema { schema_name, if_not_exists } => todo!(),
        // Statement::CreateDatabase { db_name, if_not_exists, location, managed_location } => todo!(),
        // Statement::CreateFunction { or_replace, temporary, name, args, return_type, params } => todo!(),
        // Statement::CreateProcedure { or_alter, name, params, body } => todo!(),
        // Statement::CreateMacro { or_replace, temporary, name, args, definition } => todo!(),
        // Statement::CreateStage { or_replace, temporary, if_not_exists, name, stage_params, directory_table_params, file_format, copy_options, comment } => todo!(),
        // Statement::Assert { condition, message } => todo!(),
        // Statement::Grant { privileges, objects, grantees, with_grant_option, granted_by } => todo!(),
        // Statement::Revoke { privileges, objects, grantees, granted_by, cascade } => todo!(),
        // Statement::Deallocate { name, prepare } => todo!(),
        // Statement::Execute { name, parameters } => todo!(),
        // Statement::Prepare { name, data_types, statement } => todo!(),
        // Statement::Kill { modifier, id } => todo!(),
        // Statement::ExplainTable { describe_alias, table_name } => todo!(),
        // Statement::Explain { describe_alias, analyze, verbose, statement, format } => todo!(),
        // Statement::Savepoint { name } => todo!(),
        // Statement::ReleaseSavepoint { name } => todo!(),
        // Statement::Merge { into, table, source, on, clauses } => todo!(),
        // Statement::Cache { table_flag, table_name, has_as, options, query } => todo!(),
        // Statement::UNCache { table_name, if_exists } => todo!(),
        // Statement::CreateSequence { temporary, if_not_exists, name, data_type, sequence_options, owned_by } => todo!(),
        // Statement::CreateType { name, representation } => todo!(),
        // Statement::Pragma { name, value, is_eq } => todo!()
    }
}