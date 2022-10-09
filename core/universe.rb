module Core
  class Universe
    attr_reader :number, :data

    def initialize(number)
      @number = number
      @data = Array.new(512, 0)
    end

    def apply(address, data)
      @data[address_range(address, data.length)] = data
    end

    def get(start, footprint)
      @data[address_range(start, footprint)]
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

    private

    def address_range(start, size)
      (start-1)..(start+size-1)
    end
  end
end
