pub struct TradeMapper;
pub struct PositionMapper;

pub struct NetWorkMapper;
// use super::http_data::TradeRe;
use crate::actors::database::get_connect;
// use log::info;
use mysql::*;
use mysql::prelude::*;
use serde_json::Value;
// use super::db_data::Trade;


impl TradeMapper {
  pub fn insert_incomes(incomes: Vec<Value>, name: &str) -> bool {
    let mut coon = get_connect();
    let mut value = "";


    

    if name == "Angus" {
      value = r"INSERT INTO incomes (time, type, asset, amount, tran_id, status)
      VALUES (:time, :type, :asset, :amount, :tran_id, :status)";
    } else if name == "trader02" {
      value = r"INSERT INTO incomes_2 (time, type, asset, amount, tran_id, status)
      VALUES (:time, :type, :asset, :amount, :tran_id, :status)";
    } else if name == "xh01_feng4_virtual" {
      value = r"INSERT INTO incomes_4 (time, type, asset, amount, tran_id, status)
      VALUES (:time, :type, :asset, :amount, :tran_id, :status)";
    } else if name == "xh02_b20230524_virtual" {
      value = r"INSERT INTO incomes_5 (time, type, asset, amount, tran_id, status)
      VALUES (:time, :type, :asset, :amount, :tran_id, :status)";
    } else if name == "xh03_feng3_virtual" {
      value = r"INSERT INTO incomes_6 (time, type, asset, amount, tran_id, status)
      VALUES (:time, :type, :asset, :amount, :tran_id, :status)";
    } else if name == "xh04_20230524_virtual" {
      value = r"INSERT INTO incomes_7 (time, type, asset, amount, tran_id, status)
      VALUES (:time, :type, :asset, :amount, :tran_id, :status)";
    } else if name == "trader04" {
      value = r"INSERT INTO incomes_3 (time, type, asset, amount, tran_id, status)
      VALUES (:time, :type, :asset, :amount, :tran_id, :status)";
    }

    let income = coon.exec_batch(
      value
      ,
      incomes.iter().map(|p| params! {
        "time" => &p["time"],
        "type" => &p["type"],
        "asset" => &p["asset"],
        "amount" => &p["amount"],
        "tran_id" => &p["tran_id"],
        "status" => &p["status"]
      })
    );

    match income {
      Ok(_c) => {
        println!("insert position success");
        return true;
      },
      Err(e) => {
        eprintln!("error:{}", e);
        return false;
      }
    }
  }


}










