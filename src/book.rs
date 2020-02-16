use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
  book_id: i32,
  title: String,
  title_yomi: String,
}

// ("title_sort", String("あくまのした"))
// ("subtitle", String(""))
// ("subtitle_yomi", String(""))
// ("original_title", String(""))
// ("first_appearance", String(""))
// ("ndc_code", String("NDC 913"))
// ("font_kana_type", String("新字旧仮名"))
// ("copyright", Boolean(false))
// ("release_date", UtcDatetime(1999-01-23T00:00:00Z))
// ("last_modified", UtcDatetime(2014-09-17T00:00:00Z))
// ("card_url", String("https://www.aozora.gr.jp/cards/000014/card12.html"))
// ("base_book_1", String("村山槐多全集"))
// ("base_book_1_publisher", String("彌生書房"))
// ("base_book_1_1st_edition", String(""))
// ("base_book_1_edition_input", String("1997（平成9）年3月10日増補2版"))
// ("base_book_1_edition_proofing", String(""))
// ("base_book_1_parent", String(""))
// ("base_book_1_parent_publisher", String(""))
// ("base_book_1_parent_1st_edition", String(""))
// ("base_book_2", String(""))
// ("base_book_2_publisher", String(""))
// ("base_book_2_1st_edition", String(""))
// ("base_book_2_edition_input", String(""))
// ("base_book_2_edition_proofing", String(""))
// ("base_book_2_parent", String(""))
// ("base_book_2_parent_publisher", String(""))
// ("base_book_2_parent_1st_edition", String(""))
// ("input", String("小林徹"))
// ("proofing", String("山本奈津恵"))
// ("text_url", String("https://www.aozora.gr.jp/cards/000014/files/12_ruby_22756.zip"))
// ("text_last_modified", UtcDatetime(2006-04-24T00:00:00Z))
// ("text_encoding", String("ShiftJIS"))
// ("text_charset", String("JIS X 0208"))
// ("text_updated", I32(5))
// ("html_url", String("https://www.aozora.gr.jp/cards/000014/files/12_22757.html"))
// ("html_last_modified", UtcDatetime(2006-04-24T00:00:00Z))
// ("html_encoding", String("ShiftJIS"))
// ("html_charset", String("JIS X 0208"))
// ("html_updated", I32(0))
// ("authors", Array([Document(OrderedDocument({"person_id": I32(14), "last_name": String("村山"), "first_name": String("槐多")}))]))
