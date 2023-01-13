//
//  ContentView.swift
//  Example
//
//  Created by Shota Shimazu on 2023/01/13.
//

import SwiftUI
import CoreOverlay

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
            Text("Hello, world!")
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
