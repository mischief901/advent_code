defmodule Main do

  def main1() do
    {:ok, file} = File.read("./input_3")
    scan_mult(file)
    |> inspect()
  end

  def main2() do
    {:ok, file} = File.read("./input_3")
    scan_mult_dos(file)
    |> inspect()
  end

  def scan_mult(file) do
    scan_mult(0, file)
  end

  def scan_mult(total, file) do
    ~r/mul\(([0-9]{1,3}),([0-9]{1,3})\)+/
    |> Regex.scan(file, [capture: :all])
    |> mult(total)
  end

  def mult([], total) do
    total
  end
  
  def mult([[match, value_a, value_b] | rest], total) do
    inspect(match)
    a = String.to_integer(value_a)
    b = String.to_integer(value_b)
    inspect(a * b)
    mult(rest, total + (a*b))
  end

  def scan_mult_dos(file) do
    scan_mult_dos(0, file)
  end

  def scan_mult_dos(total, file) do
    ~r/mul\(([0-9]{1,3}),([0-9]{1,3})\)+|(don't)+|(do)+/
    |> Regex.scan(file, [capture: :all])
    |> IO.inspect()
    |> mult_do(total, true)
  end


  def mult_do([], total, _) do
    total
  end
  
  def mult_do([[match, value_a, value_b] | rest], total, true) do
    IO.inspect(match)
    a = String.to_integer(value_a)
    b = String.to_integer(value_b)
    mult_do(rest, total + (a*b), true)
  end

  def mult_do([[match, _, _, "don't"] | rest], total, _) do
    IO.inspect(match)
    mult_do(rest, total, false)
  end

  def mult_do([[match, _, _, _, "do"] | rest], total, _) do
    IO.inspect(match)
    mult_do(rest, total, true)
  end

  def mult_do([_ | rest], total, false) do
    mult_do(rest, total, false)
  end
end
