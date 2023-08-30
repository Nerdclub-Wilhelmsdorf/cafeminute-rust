use serde::Deserialize;

#[derive(FromForm)]
pub struct MyForm {
    pub field: String,
}

#[derive(FromForm)]
pub struct SetOpen {
    pub pswd: String,
}
#[derive(FromForm, Clone)]
pub struct SetCustomers {
    pub pswd: String,
    pub customers: i8,
}
#[derive(FromForm, Clone)]
pub struct Schedule {
    pub pswd: String,
    pub mon: String,
    pub tue: String,
    pub wed: String,
    pub thu: String,
    pub fri: String
}


#[derive(FromForm, Clone)]

pub struct AddProduct {
    pub pswd: String,
    pub title: String,
    pub allergenic: String,
    pub prize: String,
    pub description: String,
    pub sale: String,
    pub calories: String,
    pub image: String,
}

#[derive(FromForm, Clone)]
pub struct GetProducts {
    pub id: String,
}
#[derive(FromForm, Clone)]

pub struct ChangeProduct {
    pub pswd: String,
    pub id: String,
    pub key: String,
    pub value: String,
}

#[derive(FromForm, Clone)]

pub struct RemoveProduct {
    pub pswd: String,
    pub id: String,
}
/*
type addNews struct {
	PSWD    string `json:"pswd" xml:"pswd" form:"pswd" query:"pswd"`
	HEADING string `json:"heading" xml:"heading" form:"heading" query:"heading"`
	CONTENT string `json:"content" xml:"content" form:"content" query:"content"`
	IMAGE   string `json:"image" xml:"image" form:"image" query:"image"`
	EVENT   string `json:"event" xml:"event" form:"event" query:"event"`
}
*/
#[derive(FromForm, Clone)]
pub struct AddNews {
    pub pswd: String,
    pub heading: String,
    pub content: String,
    pub image: String,
    pub event: String,
}
