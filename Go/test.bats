setup() {
    go build .
}

@test "xor" {
    run ./NeuralNetworkInAllLangs
    printf 'Lines:\n'
    printf 'lines %s\n' "${lines[@]}" >&2
    printf 'output %s\n' "${output[@]}" >&2
    [ "${lines[0]}" = "Result after 4000 iterations" ]
    [ "${lines[1]}" = "        XOR  XNOR    OR   AND   NOR  NAND" ]
    [ "${lines[2]}" = "0,0 = 0.038 0.962 0.038 0.001 0.963 0.999" ]
    [ "${lines[3]}" = "0,1 = 0.961 0.039 0.970 0.026 0.029 0.974" ]
    [ "${lines[4]}" = "1,0 = 0.961 0.039 0.970 0.026 0.030 0.974" ]
    [ "${lines[5]}" = "1,1 = 0.049 0.952 0.994 0.956 0.006 0.044" ]
    [ "${lines[6]}" = "weights hidden:" ]
    [[ "${lines[7]}" =~ '-5.942533 -7.053490' ]]
    [[ "${lines[8]}" =~ '-5.938492 -7.080670' ]]
    [ "${lines[9]}" = "biases hidden:" ]
    [[ "${lines[10]}" =~ '8.660800  3.181028' ]]
    [ "${lines[11]}" = "weights output:" ]
    [[ "${lines[12]}" =~ '7.028475 -7.038526 -1.730834 -7.403271  1.558926  7.390187' ]]
    [[ "${lines[13]}" =~ '-7.310426  7.321119 -7.038799 -2.800220  7.071517  3.219192' ]]
    [ "${lines[14]}" = "biases output:" ]
    [[ "${lines[15]}" =~ '-3.245795  3.250424  5.246738  3.363781 -5.097047 -3.361940' ]]
}
