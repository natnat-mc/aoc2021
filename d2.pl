% prolog
file_lines(File, Lines):-
	setup_call_cleanup(
		open(File, read, In),
		stream_lines(In, Lines),
		close(In)
	).
stream_lines(In, Lines):-
	read_line_to_string(In, Line),
	(
		Line == end_of_file
		->
			Lines = []
		;
			Lines = [Line | Next],
			stream_lines(In, Next)
	).

reverse(A, B):-
	reverse(A, B, []).
reverse([], Z, Z).
reverse([H|T], Z, Acc):-
	reverse(T, Z, [H|Acc]).

parse_lines([], []).
parse_lines([H|T], [Ph|Th]):-
	parse(H, Ph),
	parse_lines(T, Th).

parse(Line, [Op, Dist]):-
	split_string(Line, " ", "", [Op_str, Dist_str]),
	term_string(Op, Op_str),
	number_string(Dist, Dist_str).

part1(File, Out):-
	file_lines(File, RawLines),
	parse_lines(RawLines, Lines),
	reverse(Lines, LinesRev),
	part1(LinesRev, Horz, Vert),
	Out is Horz * Vert.

part1([], 0, 0).
part1([[forward, D] | Tail], Horz, Vert):-
	part1(Tail, Horz1, Vert),
	Horz is Horz1 + D.
part1([[up, D] | Tail], Horz, Vert):-
	part1(Tail, Horz, Vert1),
	Vert is Vert1 - D.
part1([[down, D] | Tail], Horz, Vert):-
	part1(Tail, Horz, Vert1),
	Vert is Vert1 + D.

part2(File, Out):-
	file_lines(File, RawLines),
	parse_lines(RawLines, Lines),
	reverse(Lines, LinesRev),
	part2(LinesRev, Horz, Vert, _),
	Out is Horz * Vert.

part2([], 0, 0, 0).
part2([[forward, D] | Tail], Horz, Vert, Aim):-
	part2(Tail, Horz1, Vert1, Aim),
	Horz is Horz1+D,
	Vert is Vert1+D*Aim.
part2([[up, D] | Tail], Horz, Vert, Aim):-
	part2(Tail, Horz, Vert, Aim1),
	Aim is Aim1 - D.
part2([[down, D] | Tail], Horz, Vert, Aim):-
	part2(Tail, Horz, Vert, Aim1),
	Aim is Aim1 + D.
