use serde::{Deserialize, Serialize};

use crate::user::{ImageUrl, Username};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub username: Username,
    pub bio: Option<String>,
    pub image: Option<ImageUrl>,
    pub following: bool,
}

impl Profile {
    pub fn new(
        username: Username,
        bio: Option<String>,
        image: Option<ImageUrl>,
        following: bool,
    ) -> Self {
        Self {
            username,
            bio,
            image,
            following,
        }
    }

    pub fn follow(&mut self) {
        self.following = true;
    }

    pub fn unfollow(&mut self) {
        self.following = false;
    }

    /// Validate that a follow/unfollow action is allowed
    /// Returns error if user tries to follow/unfollow themselves
    pub fn validate_follow_action(target_id: &crate::UserId, follower_id: &crate::UserId) -> crate::DomainResult<()> {
        if target_id == follower_id {
            return Err(crate::DomainError::UnauthorizedAction);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileEnvelope {
    pub profile: Profile,
}

impl From<Profile> for ProfileEnvelope {
    fn from(value: Profile) -> Self {
        Self { profile: value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user::Username;

    #[test]
    fn profile_follow_and_unfollow_toggle_flag() {
        let mut profile = Profile::new(Username::new("jake").unwrap(), None, None, false);
        profile.follow();
        assert!(profile.following);
        profile.unfollow();
        assert!(!profile.following);
    }
}
