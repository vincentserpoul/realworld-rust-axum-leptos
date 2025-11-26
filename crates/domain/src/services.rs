/// Domain services for operations that span multiple aggregates
use std::collections::{HashMap, HashSet};

use crate::{ArticleId, UserId};

/// Add a follower relationship
pub fn add_follower(
    followers: &mut HashMap<UserId, HashSet<UserId>>,
    target_id: UserId,
    follower_id: UserId,
) {
    let entry = followers.entry(target_id).or_default();
    entry.insert(follower_id);
}

/// Remove a follower relationship
pub fn remove_follower(
    followers: &mut HashMap<UserId, HashSet<UserId>>,
    target_id: UserId,
    follower_id: UserId,
) {
    if let Some(entry) = followers.get_mut(&target_id) {
        entry.remove(&follower_id);
    }
}

/// Check if a user is following another user
pub fn is_following(
    followers: &HashMap<UserId, HashSet<UserId>>,
    target_id: UserId,
    follower_id: UserId,
) -> bool {
    followers
        .get(&target_id)
        .map(|set| set.contains(&follower_id))
        .unwrap_or(false)
}

/// Check if an article is favorited by a user
pub fn is_article_favorited(
    favorites: &HashMap<ArticleId, HashSet<UserId>>,
    article_id: ArticleId,
    user_id: UserId,
) -> bool {
    favorites
        .get(&article_id)
        .map(|users| users.contains(&user_id))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_follower() {
        let mut followers = HashMap::new();
        let target_id = UserId::random();
        let follower_id = UserId::random();
        
        add_follower(&mut followers, target_id, follower_id);
        
        assert!(followers.contains_key(&target_id));
        assert!(followers.get(&target_id).unwrap().contains(&follower_id));
    }

    #[test]
    fn test_add_multiple_followers() {
        let mut followers = HashMap::new();
        let target_id = UserId::random();
        let follower1 = UserId::random();
        let follower2 = UserId::random();
        
        add_follower(&mut followers, target_id, follower1);
        add_follower(&mut followers, target_id, follower2);
        
        let set = followers.get(&target_id).unwrap();
        assert_eq!(set.len(), 2);
        assert!(set.contains(&follower1));
        assert!(set.contains(&follower2));
    }

    #[test]
    fn test_remove_follower() {
        let mut followers = HashMap::new();
        let target_id = UserId::random();
        let follower_id = UserId::random();
        
        let mut set = HashSet::new();
        set.insert(follower_id);
        followers.insert(target_id, set);
        
        remove_follower(&mut followers, target_id, follower_id);
        
        assert!(followers.get(&target_id).unwrap().is_empty());
    }

    #[test]
    fn test_remove_follower_not_following() {
        let mut followers = HashMap::new();
        let target_id = UserId::random();
        let follower_id = UserId::random();
        
        remove_follower(&mut followers, target_id, follower_id);
        // Should not panic
    }

    #[test]
    fn test_is_following_true() {
        let mut followers = HashMap::new();
        let target_id = UserId::random();
        let follower_id = UserId::random();
        
        let mut set = HashSet::new();
        set.insert(follower_id);
        followers.insert(target_id, set);
        
        assert!(is_following(&followers, target_id, follower_id));
    }

    #[test]
    fn test_is_following_false() {
        let followers = HashMap::new();
        let target_id = UserId::random();
        let follower_id = UserId::random();
        
        assert!(!is_following(&followers, target_id, follower_id));
    }

    #[test]
    fn test_is_article_favorited_true() {
        let mut favorites = HashMap::new();
        let article_id = ArticleId::random();
        let user_id = UserId::random();
        
        let mut set = HashSet::new();
        set.insert(user_id);
        favorites.insert(article_id, set);
        
        assert!(is_article_favorited(&favorites, article_id, user_id));
    }

    #[test]
    fn test_is_article_favorited_false() {
        let favorites = HashMap::new();
        let article_id = ArticleId::random();
        let user_id = UserId::random();
        
        assert!(!is_article_favorited(&favorites, article_id, user_id));
    }
}
