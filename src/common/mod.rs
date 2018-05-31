use messaging::Messager;
use microservice::*;
use diesel::SqliteConnection;
use authentication::AuthenticationService;
use authorization::AuthorizationService;
use cache::CacheAccessor;
use config::Config;

pub struct AppState {
    pub config: Config,
    pub cache: Box<CacheAccessor>,
    pub authorization: Box<AuthorizationService>,
    pub authentication: Box<AuthenticationService>,
    pub dbConnection: SqliteConnection,
    pub microservices_lifecycle: Vec<Box<LifecycleMicroservice>>,
    pub microservices_scheduled: Vec<Box<ScheduledMicroservice>>,
    pub microservices_interval: Vec<Box<FixedScheduledMicroservice>>,
    pub messengers : Vec<Box<Messager>>,
}