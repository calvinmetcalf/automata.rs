fn compute(rule:u8,value:u8)->u8{
	if((rule&(1<<value))==0){
		return 0;
	}else{
		return 1;
	}
}