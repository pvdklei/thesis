res=results.csv
if [ -f $res ]; then
	while true; do
		read -p "$res already exists, should I remove it? " yn
		case $yn in
			[Yy]*) rm results.csv; break;;
			[Nn]*) echo "Exiting..."; exit;;
			*) echo "Answer yes or no please";;
		esac 
	done
fi

for file in shaders/nm_*.glsl; do
	v="null"
	case $file in 
		*nm_rotor.glsl)
			v=rotor;;
		*nm_motor.glsl)
			v=motor;;
		*bitang*)
			v=normtang;;
		*matrix*)
			v=matrix;;
		*outer_log_motor*)
			v=outermotor;;
		*outer_log_rotor*)
			v=outerrotor;;
		*qtang*)
			v=qrotor;;
		#*cayley_motor*)
			#v=cayleymotor;;
		*cayley_rotor*)
			v=cayleyrotor;;
	esac
	if [[ "$v" == "null" ]] 
	then
		continue
	fi
	echo "[TEST] $file"
	cargo run --bin bench --release --  -w 1600 -h 1600 -p $file -v $v >/dev/null 2>&1
	if [ $? -ne 0 ]; then
		echo "[ERROR] $file"
		exit
	fi
done

python prep.py
python visual.py
