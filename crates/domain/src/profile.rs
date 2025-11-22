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
