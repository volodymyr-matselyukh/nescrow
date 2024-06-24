

// #[test]
// fn test_setup_succeeds() {
//     setup(None, None, None);
// }

// #[test]
// fn test_quest_ownership() {
//     let (mut contract, mut context, quest) = setup(None, None, None);

//     let owner_quests = contract.quests_per_owner(owner());
//     assert_eq!(owner_quests.len(), 1);

//     let first_quest = owner_quests.get(0);
//     assert_eq!(first_quest.is_some(), true);

//     let first_quest_unwrapped = first_quest.unwrap();

//     contract.set_owner(first_quest_unwrapped.quest_id, user1());

//     let owner_quests = contract.quests_per_owner(owner());
//     assert_eq!(owner_quests.len(), 0);

//     let new_owner_quests = contract.quests_per_owner(user1());
//     assert_eq!(owner_quests.len(), 0);

//     let new_owner_first_quest = new_owner_quests.get(0);
//     assert_eq!(new_owner_first_quest.is_some(), true);

//     let new_owner_first_quest_unwrapped = new_owner_first_quest.unwrap();

//     assert_eq!(&new_owner_first_quest_unwrapped.title, &quest.title);
// }