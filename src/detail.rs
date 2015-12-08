

pub struct Field{
	column: String,
	data_type: String,
	// 20 is the average text field,
	// more than 20 spans 1 more cell for 2 column cell
	// nothing spans 3 columns will make the UI looks ugly
	// more than 40 will spand 2 column and 2 rows in a multi-line entry
	data_length: usize,
	//this field must be put next to that field
	next_to: String,
	//hints or info, for tooltips
	info: String,

}

impl Field{

	
	// compute row spans and column spans base on data length
	fn compute_spans(&self){
		//two columns or three columns	
	}

}
