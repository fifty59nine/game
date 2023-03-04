use crate::logic::base::Item;

pub struct Generator;

impl Generator {
    pub fn get_items_list() -> Vec<Item> {
        vec![
            Item::new(0, "Меч", "Просто меч. Тестовий предмет", 0, 0, 1, false),
            Item::new(
                1,
                "Зілля здоров'я",
                "При використанні дає +10 ХП",
                10,
                10,
                0,
                true,
            ),
        ]
    }
}
