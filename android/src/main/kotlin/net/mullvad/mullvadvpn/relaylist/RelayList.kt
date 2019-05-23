package net.mullvad.mullvadvpn.relaylist

class RelayList {
    val countries: List<RelayCountry>

    constructor(model: net.mullvad.mullvadvpn.model.RelayList) {
        countries = model.countries.map { country ->
            val cities = country.cities.map { city -> 
                val relays = city.relays.map { relay ->
                    Relay(country.code, city.code, relay.hostname)
                }

                RelayCity(city.name, country.code, city.code, false, relays)
            }

            RelayCountry(country.name, country.code, false, cities)
        }
    }
}
