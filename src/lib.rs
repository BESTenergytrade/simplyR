pub use simplyr_lib::MarketOutput;
use simplyr_lib::{
	custom_fair_matching, pay_as_bid_matching, GridFeeMatrix, MarketInput, Order, OrderType,
};

pub fn pay_as_bid() -> MarketOutput {
	// list of orders
	let order_1 = Order {
		id: 1,
		order_type: OrderType::Ask,
		time_slot: "2022-03-04T05:06:07+00:00".to_string(),
		actor_id: "actor_1".to_string(),
		cluster_index: Some(0),
		energy_kwh: 2.0,
		price_euro_per_kwh: 0.3,
	};

	let order_2 = Order {
		id: 2,
		order_type: OrderType::Bid,
		time_slot: "2022-03-04T05:06:07+00:00".to_string(),
		actor_id: "actor_2".to_string(),
		cluster_index: Some(0),
		energy_kwh: 1.5,
		price_euro_per_kwh: 0.35,
	};

	// create a market input
	let market_input = MarketInput { orders: vec![order_1, order_2] };

	let pay_as_bid: MarketOutput = pay_as_bid_matching(&market_input);

	pay_as_bid
}

pub fn custom_fair() -> MarketOutput {
	// custom_fair_matching
	let order_1 = Order {
		id: 1,
		order_type: OrderType::Ask,
		time_slot: "2022-03-04T05:06:07+00:00".to_string(),
		actor_id: "actor_1".to_string(),
		cluster_index: Some(0),
		energy_kwh: 2.0,
		price_euro_per_kwh: 0.3,
	};

	let order_2 = Order {
		id: 2,
		order_type: OrderType::Bid,
		time_slot: "2022-03-04T05:06:07+00:00".to_string(),
		actor_id: "actor_2".to_string(),
		cluster_index: Some(0),
		energy_kwh: 1.5,
		price_euro_per_kwh: 0.35,
	};

	let grid_fee_matrix = GridFeeMatrix::from_json_str("[[0,1,1], [1,0,1], [1,1,0]]").unwrap();

	let market_input = MarketInput { orders: vec![order_1, order_2] };

	custom_fair_matching(&market_input, 1.0, &grid_fee_matrix)
}
