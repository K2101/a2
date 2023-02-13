use super::{DatabaseError, Result};
use scylla::batch::BatchStatement;
use scylla::prepared_statement::PreparedStatement;
use scylla::statement::Consistency;
use scylla::{QueryResult, Session, SessionBuilder};

#[derive(Debug)]
pub struct Prepare {
    pub insert_user_domain: PreparedStatement,
    pub insert_internal_user_domain: PreparedStatement,
    pub set_get_user_credentials: PreparedStatement,
    pub set_get_internal_user_credentials: PreparedStatement,
}

impl Prepare {
    pub fn new(
        insert_user_domain: PreparedStatement,
        insert_internal_user_domain: PreparedStatement,
        set_get_user_credentials: PreparedStatement,
        set_get_internal_user_credentials: PreparedStatement,
    ) -> Self {
        Self {
            insert_user_domain,
            insert_internal_user_domain,
            set_get_user_credentials,
            set_get_internal_user_credentials,
        }
    }
}

#[derive(Debug)]
pub struct Database {
    session: Session,
    prepare: Option<Prepare>,
}

impl Database {
    pub async fn connect(uri_1: String, uri_2: String, port: u16) -> Result<Self> {
        // temp 2
        // let username = "scylla";
        // let password = "dNE2a4P9ovXYrTb";
        // docker run -it --rm --entrypoint cqlsh scylladb/scylla -u scylla -p dNE2a4P9ovXYrTb node-0.aws_ap_southeast_1.95edbe421439ffc84b16.clusters.scylla.cloud
        // AWS_AP_SOUTHEAST_1
        // Use this name when identifying this Data Center within Scylla.
        // For example, as a parameter to NetworkTopologyStrategy when creating a keyspace.

        let url_1 = format!("{}:{}", uri_1, port);
        let url_2 = format!("{}:{}", uri_2, port);

        let consistency = Consistency::Quorum;
        let session = SessionBuilder::new()
            .known_node(url_1)
            .known_node(url_2)
            // .user(username, password)
            .default_consistency(consistency)
            .connection_timeout(std::time::Duration::from_secs(10))
            .build()
            .await?;

        println!("Database connected");
        println!("Default consistency: {:?}", consistency);
        println!("Session: {:?}", session);

        let database = Self {
            session,
            prepare: None,
        };

        Ok(database)
    }

    pub fn get_session(&self) -> &Session {
        &self.session
    }

    pub fn get_prepare(&self) -> &Prepare {
        let p = &self.prepare;
        match p {
            Some(p) => p,
            _ => unreachable!(),
        }
    }

    pub async fn database_setting(self) -> Result<Self> {
        Self::create_keyspace(&self).await?;
        Self::database_config(&self).await?;
        Self::create_table(&self).await?;
        let insert_user_domain: Result<PreparedStatement> =
            Self::set_insert_user_domain(&self).await;

        let insert_user_domain = match insert_user_domain {
            Ok(ip) => ip,
            Err(e) => return Err(e),
        };

        let insert_internal_user_domain: Result<PreparedStatement> =
            Self::set_insert_internal_user_domain(&self).await;

        let insert_internal_user_domain = match insert_internal_user_domain {
            Ok(ip) => ip,
            Err(e) => return Err(e),
        };

        let set_get_user_credentials: Result<PreparedStatement> =
            Self::set_get_user_credentials(&self).await;

        let set_get_user_credentials = match set_get_user_credentials {
            Ok(ip) => ip,
            Err(e) => return Err(e),
        };

        let set_get_internal_user_credentials: Result<PreparedStatement> =
            Self::set_get_internal_user_credentials(&self).await;

        let set_get_internal_user_credentials = match set_get_internal_user_credentials {
            Ok(ip) => ip,
            Err(e) => return Err(e),
        };

        Ok(Self {
            session: self.session,
            prepare: Some(Prepare::new(
                insert_user_domain,
                insert_internal_user_domain,
                set_get_user_credentials,
                set_get_internal_user_credentials,
            )),
        })
    }

    async fn create_keyspace(&self) -> Result<QueryResult> {
        // let keyspace = self
        //     .get_session()
        //     .query(
        //         "CREATE KEYSPACE IF NOT EXISTS auth WITH REPLICATION =
        // {'class': 'NetworkTopologyStrategy',
        //          'AWS_AP_SOUTHEAST_1': 3, 'replication_factor' : 3}",
        //         &[],
        //     )
        //     .await?;
        let keyspace = self
            .get_session()
            .query(
                "CREATE KEYSPACE IF NOT EXISTS auth WITH REPLICATION =
        {'class': 'SimpleStrategy',
                  'replication_factor' : 1}",
                &[],
            )
            .await?;

        println!("Keyspace created");
        Ok(keyspace)
    }

    async fn database_config(&self) -> Result<()> {
        let session = self.get_session();
        session.use_keyspace("auth", false).await?;
        println!("Use auth keyspace");

        // user UDT
        // session
        //     .query(
        //         "CREATE TYPE IF NOT EXISTS auth.user(
        //          role text,
        //          status text,
        //         )",
        //         &[],
        //     )
        //     .await?;

        // // internal user UDT
        // session
        //     .query(
        //         "CREATE TYPE IF NOT EXISTS auth.internal_user(
        //             role text,
        //             status text,
        //          )",
        //         &[],
        //     )
        //     .await?;

        Ok(())
    }

    async fn create_table(&self) -> Result<QueryResult> {
        let session = self.get_session();
        session
            .query(
                "CREATE TABLE IF NOT EXISTS web_user (
                customer_id uuid,
                email text,
                password text,
                phone text,
                role text,
                status text,
                PRIMARY KEY (email)
            );",
                &[],
            )
            .await?;

        let result = session
            .query(
                "CREATE TABLE IF NOT EXISTS web_internal_user (
                    employee_id uuid,
                    email text,
                    password text,
                    phone text,
                    role text,
                    status text,
                    PRIMARY KEY (email)
                );",
                &[],
            )
            .await?;

        println!("Table created");
        Ok(result)
    }

    async fn set_insert_user_domain(&self) -> Result<PreparedStatement> {
        let session = self.get_session();
        let insert = session
            .prepare(
                "INSERT INTO web_user (
                    customer_id,
                    email,
                    password,
                    phone,
                    role,
                    status
            ) VALUES(?,?,?,?,?,?);",
            )
            .await;

        let insert = match insert {
            Ok(insert) => insert,
            Err(e) => return Err(DatabaseError::QueryError(e)),
        };

        println!("insert_user_domain created");
        Ok(insert)
    }

    async fn set_insert_internal_user_domain(&self) -> Result<PreparedStatement> {
        let session = self.get_session();
        let insert_internal_user_domain = session
            .prepare(
                "INSERT INTO web_internal_user (
                    employee_id,
                    email,
                    password,
                    phone,
                    role,
                    status
                ) VALUES(?,?,?,?,?,?);",
            )
            .await;

        let insert_internal_user_domain = match insert_internal_user_domain {
            Ok(insert) => insert,
            Err(e) => return Err(DatabaseError::QueryError(e)),
        };

        println!("insert_internal_user_domain created");
        Ok(insert_internal_user_domain)
    }

    async fn set_get_user_credentials(&self) -> Result<PreparedStatement> {
        let session = self.get_session();
        let set_get_user_credentials = session
            .prepare("SELECT * FROM auth.web_user WHERE email = ?;")
            .await;

        let set_get_user_credentials = match set_get_user_credentials {
            Ok(insert) => insert,
            Err(e) => return Err(DatabaseError::QueryError(e)),
        };

        println!("set_get_user_credentials created");
        Ok(set_get_user_credentials)
    }

    async fn set_get_internal_user_credentials(&self) -> Result<PreparedStatement> {
        let session = self.get_session();
        let set_get_internal_user_credentials = session
            .prepare("SELECT * FROM auth.web_internal_user WHERE email = ?;")
            .await;

        let set_get_internal_user_credentials = match set_get_internal_user_credentials {
            Ok(insert) => insert,
            Err(e) => return Err(DatabaseError::QueryError(e)),
        };

        println!("set_get_internal_user_credentials created");
        Ok(set_get_internal_user_credentials)
    }

    // async fn set_update_prepare_statement(&self) -> Result<PreparedStatement> {
    //     let session = self.get_session();
    //     let update = session
    //         .prepare(
    //             "UPDATE retail_customer SET
    //         address = ?,
    //         current_address = ?,
    //         email = ?,
    //         is_active = ?,
    //         name_eng = ?,
    //         name = ?,
    //         phone = ?,
    //         same_address = ?,
    //         submit_date = ?,
    //         surname_eng = ?,
    //         surname = ?
    //             WHERE customer_id = ?;",
    //         )
    //         .await;

    //     let update = match update {
    //         Ok(update) => update,
    //         Err(e) => return Err(DatabaseError::QueryError(e)),
    //     };

    //     println!("Update prepare Statement created");
    //     Ok(update)
    // }
}
