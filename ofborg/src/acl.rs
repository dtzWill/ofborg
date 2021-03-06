
pub struct ACL {
    trusted_users: Vec<String>,
    known_users: Vec<String>,
}

impl ACL {
    pub fn new(trusted_users: Vec<String>, known_users: Vec<String>) -> ACL {
        return ACL {
            trusted_users: trusted_users,
            known_users: known_users,
        };
    }

    pub fn build_job_destinations_for_user_repo(
        &self,
        user: &str,
        repo: &str,
    ) -> Vec<(Option<String>, Option<String>)> {
        if self.can_build_unrestricted(user, repo) {
            vec![(Some("build-jobs".to_owned()), None)]
        } else if self.can_build_restricted(user, repo) {
            vec![
                (None, Some("build-inputs-x86_64-linux".to_owned())),
                (None, Some("build-inputs-aarch64-linux".to_owned())),
            ]
        } else {
            vec![]
        }
    }

    pub fn can_build_restricted(&self, user: &str, repo: &str) -> bool {
        if repo.to_lowercase() != "nixos/nixpkgs" {
            return false;
        }

        return self.known_users.contains(&user.to_lowercase());
    }

    pub fn can_build_unrestricted(&self, user: &str, repo: &str) -> bool {
        if repo.to_lowercase() != "nixos/nixpkgs" {
            return false;
        }

        return self.trusted_users.contains(&user.to_lowercase());
    }
}
