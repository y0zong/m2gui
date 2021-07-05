//
//  apptplApp.swift
//  Shared
//
//  Created by Orlo Wang on 2021/7/3.
//

import SwiftUI

@main
struct apptplApp: App {
    let persistenceController = PersistenceController.shared

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environment(\.managedObjectContext, persistenceController.container.viewContext)
        }
    }
}
