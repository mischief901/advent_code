defmodule Main do
  def main1() do
    {:ok, file} = File.read("input_test")
    file
    |> String.split()
    |> then(fn x -> {x, 0} end)
    |> check_rows_forward
    |> check_rows_backward
    |> check_cols
    |> check_diags
    |> then(fn {_, total} -> total end)
  end

  def main() do
    {:ok, file} = File.read("input_4")
    IO.write(file)
    IO.write("\n\n\n")
    file
    |> String.split()
    |> then(fn x -> {x, 0} end)
    |> check_xmas_forward()
    |> check_xmas_cols()
    |> check_xmas_backwards()
    |> check_xmas_upsidedown()
    |> then(fn {_, total} -> total end)
  end

  def check_xmas_forward({input, total}) do
    total =
      input
      |> Enum.chunk_every(3, 1, :discard)
      |> Enum.reduce(total, fn triple, total -> check_xmas_forward(triple, total) end)
    {input, total}
  end

  def check_xmas_forward([], total) do
    total
  end
  def check_xmas_forward([<<"M", a::binary-size(1), "S">> <> rest1,
                          <<_::binary-size(1), "A", b::binary-size(1)>> <> rest2,
                          <<"M", c::binary-size(1), "S">> <> rest3 | rest], total) do
    check_xmas_forward([a <> <<"S">> <> rest1, <<"A">> <> b <> rest2, c <> "S" <> rest3 | rest], total + 1)
  end

  def check_xmas_forward([x, y, z | rest], total) when byte_size(x) < 3 or byte_size(y) < 3 or byte_size(z) < 3 do
    check_xmas_forward(rest, total)
  end

  def check_xmas_forward([<<_::binary-size(1)>> <> rest1,
                          <<_::binary-size(1)>> <> rest2,
                          <<_::binary-size(1)>> <> rest3 | rest], total) do
    check_xmas_forward([rest1, rest2, rest3 | rest], total)
  end

  def check_xmas_cols({input, total}) do
    transformed = transform_cols(input)
    pp(transformed)
    check_xmas_forward({transformed, total})
  end

  def check_xmas_backwards({input, total}) do
    transformed = input |> Enum.map(&String.reverse/1)
    pp(transformed)
    check_xmas_forward({transformed, total})
  end

  def check_xmas_upsidedown({input, total}) do
    transformed = input |> transform_cols() |> Enum.map(&String.reverse/1)
    pp(transformed)
    check_xmas_forward({transformed, total})
  end

  def pp(input) do
    input
    |> Enum.join("\n")
    |> IO.write()
    IO.write("\n\n")
  end
  
  def check_rows_forward({test, total}) do
    total = Enum.reduce(test, total, fn test, total -> check_rows_forward(test, total) end)
    {test, total}
  end

  def check_rows_forward(<<>>, total) do
    total
  end
  def check_rows_forward(<<"XMAS">> <> rest, total) do
    check_rows_forward(rest, total + 1)
  end
  def check_rows_forward(<<_::binary-size(1)>> <> rest, total) do
    check_rows_forward(rest, total)
  end

  def check_rows_backward({test, total}) do
    total = Enum.reduce(test, total, fn test, total -> check_rows_backward(test, total) end)
    {test, total}
  end

  def check_rows_backward(<<>>, total) do
    total
  end
  def check_rows_backward(<<"SAMX">> <> rest, total) do
    check_rows_backward(rest, total + 1)
  end
  def check_rows_backward(<<_::binary-size(1)>> <> rest, total) do
    check_rows_backward(rest, total)
  end

  def check_cols({test, total}) do
    transformed = transform_cols(test)
    {_, total} = check_rows_forward({transformed, total})
    {_, total} = check_rows_backward({transformed, total})
    {test, total}
  end

  def transform_cols(current) do
    transform_cols(current, [])
  end

  def transform_cols([head | rest], []) do
    transform_cols(rest, String.split(head, "", trim: true))
  end
  def transform_cols([], acc) do
    acc
  end
  def transform_cols([next | rest], acc) do
    acc = Enum.zip_with([String.split(next, "", trim: true), acc], fn [n, acc] -> acc <> n end)
    transform_cols(rest, acc)
  end

  def check_diags({test, total}) do
    transformed = transform_diag_right(test) |> Enum.flat_map(fn x -> x end)
    
    {_, total} = check_rows_forward({transformed, total})
    {_, total} = check_rows_backward({transformed, total})

    transformed = transform_diag_left(test) |> Enum.flat_map(fn x -> x end)
    
    {_, total} = check_rows_forward({transformed, total})
    {_, total} = check_rows_backward({transformed, total})
    {test, total}
  end
  
  def check_diag_forward_right({test,total}) do
    transformed = transform_diag_right(test)
    check_rows_forward({transformed, total})
  end
  def check_diag_backward_right({test,total}) do
    transformed = transform_diag_right(test)
    check_rows_backward({transformed, total})
  end

  def transform_diag_right(current) do
    current
    |> Enum.map(&String.split(&1, "", trim: true))
    |> transform_diag_right([])
  end

  def transform_diag_right(
    [[a | rest1],
     [_, f | rest2] = full2,
     [_, _, k | rest3] = full3,
     [_, _, _, p | rest4] = full4 | rest],
    []) do
    transform_diag_right([rest1, rest2, rest3, rest4 | rest], [Enum.join([a, f, k, p])], full2, full3, full4)
  end

  def transform_diag_right([_, _, _ | []], acc) do
    acc
  end

  def transform_diag_right([_, _, _, [] | rest], acc, full2, full3, full4) do
    [acc | transform_diag_right([full2, full3, full4 | rest], [])]
  end
  def transform_diag_right(
    [[a | rest1],
     [f | rest2],
     [k | rest3],
     [p | rest4] | rest],
    acc, full2, full3, full4) do

    transform_diag_right([rest1, rest2, rest3, rest4 | rest], acc ++ [Enum.join([a, f, k, p])], full2, full3, full4)
  end
  def transform_diag_right([_, _, _ | []], acc, _, _, _) do
    acc
  end
  
  def transform_diag_left(current) do
    current
    |> Enum.map(&String.split(&1, "", trim: true))
    |> transform_diag_left([])
  end

  def transform_diag_left(
    [[_, _, _, a | rest1],
     [_, _, f | rest2] = full2,
     [_, k | rest3] = full3,
     [p | rest4] = full4 | rest],
    []) do
    transform_diag_left([rest1, rest2, rest3, rest4 | rest], [Enum.join([a, f, k, p])], full2, full3, full4)
  end

  def transform_diag_left([_, _, _ | []], acc) do
    acc
  end

  def transform_diag_left([[],_,_,_ | rest], acc, full2, full3, full4) do
    [acc | transform_diag_left([full2, full3, full4 | rest], [])]
  end
  def transform_diag_left(
    [[a | rest1],
     [f | rest2],
     [k | rest3],
     [p | rest4] | rest],
    acc, full2, full3, full4) do

    transform_diag_left([rest1, rest2, rest3, rest4 | rest], acc ++ [Enum.join([a, f, k, p])], full2, full3, full4)
  end
  def transform_diag_left([_, _, _ | []], acc, _, _, _) do
    acc
  end
end


