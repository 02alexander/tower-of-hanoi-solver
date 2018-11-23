
pub fn move_n_disks(n_disks: u32, src: usize, dst: usize) -> Vec<(usize,usize)> {
	let mut moves = Vec::new();
	assert!(src!=dst);
	if n_disks != 0 {
		// tower that is not src or dst
		let tmp_twr = if (src==0 && dst==2) || (src==2 && dst==0) {
			1
		} else if (src==1 && dst==2) || (src==2 && dst==1) {
			0
		} else {
			2
		};
		moves.append(&mut move_n_disks(n_disks-1, src, tmp_twr));
		moves.push((src,dst));
		moves.append(&mut move_n_disks(n_disks-1,tmp_twr,dst));
	}
	return moves;
}