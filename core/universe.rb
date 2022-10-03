module Core
  class Universe
    attr_reader :number, :data

    def initialize(number)
      @number = number
      @data = Array.new(512, 0)
    end

    def apply(address, data)
      # Addresses are 1 indexed, not 0
      address = address - 1
      @data[address..(address+data.length-1)] = data
    end

    def dump
      puts "-"*4*32
      puts "Universe #{@number} - #{@data.length}"
      puts "-"*4*32

      @data.each_with_index do |value, index|
        printf("%03d ", value)
        print("\n") if (index + 1) % 32 == 0
      end
    end
  end
end
