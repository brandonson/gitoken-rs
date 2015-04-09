pub enum Scope {
  User,
  UserEmail,
  UserFollow,
  PublicRepo,
  Repo,
  RepoDeployment,
  RepoStatus,
  DeleteRepo,
  Notifications,
  Gist,
  ReadRepoHook,
  WriteRepoHook,
  AdminRepoHook,
  AdminOrgHook,
  ReadOrg,
  WriteOrg,
  AdminOrg,
  ReadPublicKey,
  WritePublicKey,
  AdminPublicKey
}

impl Scope {
  pub fn token_scope_string(&self) -> String {
    match *self {
      Scope::User => "user",
      Scope::UserEmail => "user:email",
      Scope::UserFollow => "user:follow",
      Scope::PublicRepo => "public_repo",
      Scope::Repo => "repo",
      Scope::RepoDeployment => "repo_deployment",
      Scope::RepoStatus => "repo:status",
      Scope::DeleteRepo => "delete_repo",
      Scope::Notifications => "notifications",
      Scope::Gist => "gist",
      Scope::ReadRepoHook => "read:repo_hook",
      Scope::WriteRepoHook => "write:repo_hook",
      Scope::AdminRepoHook => "admin:repo_hook",
      Scope::AdminOrgHook => "admin:org_hook",
      Scope::ReadOrg => "read:org",
      Scope::WriteOrg => "write:org",
      Scope::AdminOrg => "admin:org",
      Scope::ReadPublicKey => "read:public_key",
      Scope::WritePublicKey => "write:public_key",
      Scope::AdminPublicKey => "admin:public_key"
    }.to_string()
  }
}
